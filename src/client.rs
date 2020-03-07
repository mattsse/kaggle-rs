use std::convert::TryInto;
use std::fmt;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;

use bytes::Bytes;
use futures::stream::{self, Stream, StreamExt, TryStreamExt};
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::{IntoUrl, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncRead;
use tokio_util::codec;

use anyhow::{anyhow, Context};

use crate::models::extended::{File, LeaderboardEntry, Submission, SubmitResult};
use crate::models::{
    DatasetNewRequest,
    DatasetNewVersionRequest,
    DatasetUpdateSettingsRequest,
    KernelPushRequest,
};
use crate::request::CompetitionsList;

/// Describes API errors
#[derive(Debug)]
pub enum ApiError {
    Unauthorized,
    RateLimited(Option<usize>),
    Other(u16),
}

impl std::error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::Unauthorized => write!(f, "Unauthorized request to API"),
            ApiError::RateLimited(e) => {
                if let Some(d) = e {
                    write!(f, "Exceeded API request limit - please wait {} seconds", d)
                } else {
                    write!(f, "Exceeded API request limit")
                }
            }
            ApiError::Other(s) => write!(f, "Kaggle API reported error code {}", s),
        }
    }
}

#[derive(Clone)]
pub struct KaggleApiClient {
    client: Rc<reqwest::Client>,
    base_url: Url,
    credentials: KaggleCredentials,
}

impl KaggleApiClient {
    const HEADER_API_VERSION: &'static str = "X-Kaggle-ApiVersion";

    /// Convenience method to create a [`KaggleApiClientBuilder`]
    #[inline]
    pub fn builder() -> KaggleApiClientBuilder {
        KaggleApiClientBuilder::default()
    }
}

#[derive(Debug, Clone)]
pub struct KaggleApiClientBuilder {
    base_url: Url,
    user_agent: Option<String>,
    client: Option<Rc<reqwest::Client>>,
    headers: Option<HeaderMap>,
    auth: Option<Authentication>,
}

impl KaggleApiClientBuilder {
    fn default_headers() -> HeaderMap {
        let headers = HeaderMap::with_capacity(3);
        // TODO do i need this at all?
        headers
    }

    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        if self.headers.is_none() {
            self.headers = Some(Self::default_headers());
        }
        self.headers.as_mut().unwrap()
    }

    pub fn user_agent<T: ToString>(mut self, user_agent: T) -> Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }

    pub fn client(mut self, client: Rc<reqwest::Client>) -> Self {
        self.client = Some(client);
        self
    }

    pub fn auth(mut self, auth: Authentication) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn build(self) -> anyhow::Result<KaggleApiClient> {
        let credentials = self
            .auth
            .unwrap_or_else(|| Authentication::default())
            .credentials()?;

        let mut headers = self.headers.unwrap_or_else(|| Self::default_headers());

        let mut header_value = b"Basic ".to_vec();
        {
            // See [`reqwest::Request`]
            let mut encoder =
                base64::write::EncoderWriter::new(&mut header_value, base64::STANDARD);
            write!(encoder, "{}:", &credentials.user_name)?;
            write!(encoder, "{}", &credentials.key)?;
        }

        headers.insert(header::AUTHORIZATION, header_value.try_into()?);
        if let Some(user_agent) = self.user_agent {
            headers.insert(header::USER_AGENT, user_agent.parse()?);
        } else {
            headers.insert(
                header::USER_AGENT,
                HeaderValue::from_static("kaggele-rs/1/rust"),
            );
        }
        // TODO json default?
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        let client = if let Some(client) = self.client {
            client
        } else {
            Rc::new(
                reqwest::Client::builder()
                    .default_headers(headers)
                    .build()?,
            )
        };

        Ok(KaggleApiClient {
            client,
            base_url: self.base_url,
            credentials,
        })
    }
}

impl Default for KaggleApiClientBuilder {
    fn default() -> Self {
        Self {
            base_url: "https://www.kaggle.com/api/v1".parse().unwrap(),
            user_agent: None,
            client: None,
            headers: None,
            auth: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KaggleCredentials {
    user_name: String,
    key: String,
}

impl KaggleCredentials {
    fn from_env() -> anyhow::Result<Self> {
        let user_name = std::env::var("KAGGLE_USERNAME")
            .context("KAGGLE_USERNAME env variable not present.")?;
        let key = std::env::var("KAGGLE_KEY").context("KAGGLE_KEY env variable not present.")?;
        Ok(KaggleCredentials { user_name, key })
    }

    fn from_default_json() -> anyhow::Result<Self> {
        if let Ok(path) = std::env::var("KAGGLE_CONFIG_DIR") {
            Self::from_json(path)
        } else {
            Self::from_json(
                dirs::home_dir()
                    .map(|p| p.join(".kaggle/kaggle.json"))
                    .context("Failed to detect home directory.")?,
            )
        }
    }

    fn from_json<T: AsRef<Path>>(path: T) -> anyhow::Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            Err(anyhow!(
                "kaggle config file {} does not exist",
                path.display()
            ))
        } else {
            let content = std::fs::read_to_string(path)?;
            Ok(serde_json::from_str(&content)?)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Authentication {
    /// Get the credentials from `KAGGLE_USERNAME` and `KAGGLE_KEY` env
    /// variables.
    Env,
    ConfigFile {
        /// Where the `kaggle.json` file is stored.
        /// Default location is `~/.kaggle/kaggle.json` and on windows
        /// `C:\Users\<Windows-username>\.kaggle\kaggle.json`
        path: Option<PathBuf>,
    },
    /// Use dedicated credentials for authentication.
    Credentials { user_name: String, key: String },
}

impl Authentication {
    pub fn with_credentials<S: ToString, T: ToString>(user_name: S, key: T) -> Self {
        Authentication::Credentials {
            user_name: user_name.to_string(),
            key: key.to_string(),
        }
    }
}

impl Authentication {
    fn credentials(self) -> anyhow::Result<KaggleCredentials> {
        match self {
            Authentication::Env => KaggleCredentials::from_env(),
            Authentication::ConfigFile { path } => {
                if let Some(path) = path {
                    KaggleCredentials::from_json(path)
                } else {
                    KaggleCredentials::from_default_json()
                }
            }
            Authentication::Credentials { user_name, key } => {
                Ok(KaggleCredentials { user_name, key })
            }
        }
    }
}

impl Default for Authentication {
    fn default() -> Self {
        Authentication::ConfigFile { path: None }
    }
}

pub struct ApiResp;

impl KaggleApiClient {
    async fn get<U: IntoUrl>(&self, url: U) -> anyhow::Result<String> {
        Ok(Self::request(self.client.get(url)).await?.text().await?)
    }

    async fn post_json<T: DeserializeOwned, U: IntoUrl, B: Into<reqwest::Body>>(
        &self,
        url: U,
        body: Option<B>,
    ) -> anyhow::Result<T> {
        let mut req = self.client.post(url);
        if let Some(body) = body {
            req = req.body(body);
        }
        Ok(Self::request_json(req).await?)
    }

    async fn get_json<T: DeserializeOwned, U: IntoUrl>(&self, url: U) -> anyhow::Result<T> {
        Ok(Self::request_json(self.client.get(url)).await?)
    }

    async fn request_json<T: DeserializeOwned>(req: reqwest::RequestBuilder) -> anyhow::Result<T> {
        Ok(Self::request(req).await?.json::<T>().await?)
    }

    /// Execute the request.
    async fn request(req: reqwest::RequestBuilder) -> anyhow::Result<reqwest::Response> {
        let resp = req.send().await?;

        if resp.status().is_success() {
            Ok(resp)
        } else {
            let err = match resp.status() {
                StatusCode::UNAUTHORIZED => ApiError::Unauthorized,
                StatusCode::TOO_MANY_REQUESTS => {
                    if let Ok(duration) = resp.headers()[reqwest::header::RETRY_AFTER].to_str() {
                        ApiError::RateLimited(duration.parse::<usize>().ok())
                    } else {
                        ApiError::RateLimited(None)
                    }
                }
                status => ApiError::Other(status.as_u16()),
            };
            Err(err)?
        }
    }

    fn join_url<T: AsRef<str>>(&self, path: T) -> anyhow::Result<Url> {
        Ok(self.base_url.join(path.as_ref())?)
    }
}

impl KaggleApiClient {
    /// Returns a list of `Competition'  instances.
    pub async fn competitions_list(
        &self,
        competition: CompetitionsList,
    ) -> anyhow::Result<ApiResp> {
        let req = self
            .client
            .get(self.join_url("competitions/list")?)
            .query(&competition);
        unimplemented!("Not implemented yet.")
    }

    pub async fn competition_download_leaderboard(&self, _id: &str) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    /// View a leaderboard based on a competition name
    pub async fn competition_view_leaderboard(
        &self,
        id: &str,
    ) -> anyhow::Result<Vec<LeaderboardEntry>> {
        let req = self
            .client
            .get(self.join_url(format!("/competitions/{}/leaderboard/view", id))?);
        unimplemented!("Not implemented yet.")
    }

    ///
    pub async fn competitions_data_download_file(
        &self,
        _id: &str,
        _file_name: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    ///
    pub async fn competitions_data_download_files(&self, _id: &str) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    ///
    pub async fn competitions_data_list_files(&self, id: &str) -> anyhow::Result<Vec<File>> {
        let req = self
            .client
            .get(self.join_url(format!("/competitions/data/list/{}", id))?);
        unimplemented!("Not implemented yet.")
    }

    /// Get the list of Submission for a particular competition
    pub async fn competitions_submissions_list(
        &self,
        id: &str,
        page: usize,
    ) -> anyhow::Result<Vec<Submission>> {
        let req = self
            .client
            .get(self.join_url(format!("/competitions/submissions/list/{}", id))?)
            .query(&[("page", page)]);

        unimplemented!("Not implemented yet.")
    }

    ///
    pub async fn competitions_submissions_submit(
        &self,
        _id: &str,
        _blob_file_tokens: &str,
        _submission_description: Option<String>,
    ) -> anyhow::Result<SubmitResult> {
        // last modified: Return the last modification time of a file,
        // content_length: size of the file

        // TODO call self.competitions_submissions_url
        unimplemented!("Not implemented yet.")
    }

    /// Submit a competition
    pub async fn competition_submit<S: AsRef<Path>, T: AsRef<str>>(
        &self,
        file: S,
        competition: T,
        message: Option<String>,
    ) -> anyhow::Result<SubmitResult> {
        let competition = competition.as_ref();
        let file = file.as_ref();
        let meta = file.metadata()?;
        let content_length = meta.len();
        let last_modified = meta
            .modified()
            .unwrap_or_else(|_| std::time::SystemTime::now())
            .elapsed()?;

        let file_name = file
            .file_name()
            .context("File path terminates in `..`")?
            .to_str()
            .context("File name is not valid unicode")?;

        let url_result = self
            .competitions_submissions_url(&competition, content_length, last_modified, file_name)
            .await?;

        let obj = url_result
            .as_object()
            .context("Expected json response object")?;

        // Temporary hack, `isComplete` exists on the old DTO but not the new,
        let upload_result = if obj.get("isComplete").is_some() {
            // old submissions path
            let url_list = obj
                .get("createUrl")
                .and_then(serde_json::Value::as_str)
                .context("Missing `createUrl` field")?;
            let parts: Vec<_> = url_list.split('/').rev().collect();
            if parts.len() < 3 {
                return Err(anyhow!(
                    "createUrl response with incomplete segments {}",
                    url_list
                ));
            }
            self.competitions_submissions_upload(
                file,
                parts[0],
                parts[1].parse()?,
                Duration::from_secs(parts[2].parse()?),
            )
            .await?
        } else {
            self.upload_complete(
                file,
                obj.get("createUrl")
                    .and_then(serde_json::Value::as_str)
                    .context("Missing createUrl in response")?,
            )
            .await?
        };

        let token = upload_result
            .as_object()
            .and_then(|x| x.get("token"))
            .and_then(serde_json::Value::as_str)
            .context("Missing upload token")?;

        Ok(self
            .competitions_submissions_submit(competition.as_ref(), token, message)
            .await?)
    }

    pub async fn upload_complete<T: AsRef<Path>, U: IntoUrl>(
        &self,
        file: T,
        url: U,
    ) -> anyhow::Result<serde_json::Value> {
        let stream = into_bytes_stream(tokio::fs::File::open(file).await?);

        let req = self
            .client
            .put(url)
            .body(reqwest::Body::wrap_stream(stream));

        unimplemented!()
    }

    /// Upload competition submission file
    pub async fn competitions_submissions_upload<T: AsRef<Path>>(
        &self,
        _file: T,
        _guid: &str,
        _content_length: u64,
        _last_modified_date_utc: Duration,
    ) -> anyhow::Result<serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }

    /// Generate competition submission URL
    pub async fn competitions_submissions_url<S: AsRef<str>, T: ToString>(
        &self,
        id: S,
        content_length: u64,
        last_modified_date_utc: Duration,
        file_name: T,
    ) -> anyhow::Result<serde_json::Value> {
        let form = reqwest::multipart::Form::new().text("fileName", file_name.to_string());

        let req = self
            .client
            .post(self.join_url(format!(
                "/competitions/{}/submissions/url/{}/{}",
                id.as_ref(),
                content_length,
                last_modified_date_utc.as_secs()
            ))?)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("multipart/form-data"),
            )
            .multipart(form);
        Ok(Self::request_json(req).await?)
    }

    pub async fn datasets_create_new(
        &self,
        _dataset_req: DatasetNewRequest,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    ///
    pub async fn datasets_create_version(
        &self,
        _owner_slug: &str,
        _dataset_slug: &str,
        _dataset_new_version_request: DatasetNewVersionRequest,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    pub async fn datasets_create_version_by_id(
        &self,
        _id: i32,
        _dataset_req: DatasetNewVersionRequest,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    pub async fn datasets_download(
        &self,
        _owner_slug: &str,
        _dataset_slug: &str,
        _dataset_version_number: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    pub async fn datasets_download_file(
        &self,
        _owner_slug: &str,
        _dataset_slug: &str,
        _file_name: &str,
        _dataset_version_number: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    pub async fn datasets_list(
        &self,
        _group: &str,
        _sort_by: &str,
        _size: &str,
        _filetype: &str,
        _license: &str,
        _tagids: &str,
        _search: &str,
        _user: &str,
        _page: usize,
        _max_size: i64,
        _min_size: i64,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    pub async fn datasets_list_files(
        &self,
        _owner_slug: &str,
        _dataset_slug: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_status(
        &self,
        _owner_slug: &str,
        _dataset_slug: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    pub async fn datasets_upload_file(
        &self,
        _file_name: &str,
        _content_length: i32,
        _last_modified_date_utc: i32,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    pub async fn datasets_view(
        &self,
        _owner_slug: &str,
        _dataset_slug: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    pub async fn kernel_output(
        &self,
        _user_name: &str,
        _kernel_slug: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    pub async fn kernel_pull(
        &self,
        _user_name: &str,
        _kernel_slug: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn kernel_push(
        &self,
        _kernel_push_request: KernelPushRequest,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    pub async fn kernel_status(
        &self,
        _user_name: &str,
        _kernel_slug: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    pub async fn kernels_list(
        &self,
        _page: usize,
        _page_size: i32,
        _search: &str,
        _group: &str,
        _user: &str,
        _language: &str,
        _kernel_type: &str,
        _output_type: &str,
        _sort_by: &str,
        _dataset: &str,
        _competition: &str,
        _parent_kernel: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    pub async fn metadata_get(
        &self,
        _owner_slug: &str,
        _dataset_slug: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    pub async fn metadata_post(
        &self,
        _owner_slug: &str,
        _dataset_slug: &str,
        _settings: DatasetUpdateSettingsRequest,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kaggle() -> KaggleApiClient {
        KaggleApiClient::builder()
            .auth(Authentication::with_credentials("name", "key"))
            .build()
            .unwrap()
    }

    #[test]
    fn competition_query() {
        let kaggle = kaggle();

        let req = kaggle
            .client
            .get(kaggle.join_url("competitions/list").unwrap())
            .query(&CompetitionsList::default())
            .build()
            .unwrap();

        assert_eq!(
            *req.url(),
            format!(
                "{}?group=&category=&sortBy=&page=1&search=",
                kaggle.join_url("competitions/list").unwrap()
            )
            .parse()
            .unwrap()
        )
    }
}

fn into_byte_stream<R>(r: R) -> impl Stream<Item = tokio::io::Result<u8>>
where
    R: AsyncRead,
{
    codec::FramedRead::new(r, codec::BytesCodec::new())
        .map_ok(|bytes| stream::iter(bytes).map(Ok))
        .try_flatten()
}

fn into_bytes_stream<R>(r: R) -> impl Stream<Item = tokio::io::Result<Bytes>>
where
    R: AsyncRead,
{
    codec::FramedRead::new(r, codec::BytesCodec::new()).map_ok(|bytes| bytes.freeze())
}
