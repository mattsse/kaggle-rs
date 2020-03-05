use crate::config::Config;
use crate::models::{
    DatasetNewRequest,
    DatasetNewVersionRequest,
    DatasetUpdateSettingsRequest,
    KernelPushRequest,
};
use crate::request::CompetitionsList;
use anyhow::{anyhow, Context};
use reqwest::header::{self, HeaderMap};
use reqwest::{IntoUrl, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;

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
    config: Config,
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

#[derive(Debug, Clone, Default)]
pub struct KaggleApiClientBuilder {
    config: Option<Config>,
    client: Option<Rc<reqwest::Client>>,
    headers: Option<HeaderMap>,
    auth: Option<Authentication>,
}

impl KaggleApiClientBuilder {
    fn default_headers() -> HeaderMap {
        let mut headers = HeaderMap::with_capacity(3);

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

    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
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
        let config = self.config.unwrap_or_default();

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
        headers.insert(header::USER_AGENT, config.user_agent.parse()?);
        // TODO json default?
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
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
            config,
            credentials,
        })
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
        Ok(Self::request(req).await?.json::<T>().await?)
    }

    async fn get_json<T: DeserializeOwned, U: IntoUrl>(&self, url: U) -> anyhow::Result<T> {
        Ok(Self::request(self.client.get(url))
            .await?
            .json::<T>()
            .await?)
    }

    async fn request(mut req: reqwest::RequestBuilder) -> anyhow::Result<reqwest::Response> {
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
        Ok(self.config.base_url.join(path.as_ref())?)
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
            .query(&[competition]);
        unimplemented!("Not implemented yet.")
    }

    pub async fn competition_download_leaderboard(&self, id: &str) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    pub async fn competition_view_leaderboard(&self, id: &str) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    pub async fn competitions_data_download_file(
        &self,
        id: &str,
        file_name: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn competitions_data_download_files(&self, id: &str) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn competitions_data_list_files(&self, id: &str) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    /// Get the list of Submission for a particular competition
    pub async fn competitions_submissions_list(
        &self,
        id: &str,
        page: usize,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn competitions_submissions_submit(
        &self,
        blob_file_tokens: &str,
        submission_description: &str,
        id: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn competitions_submissions_upload(
        &self,
        file: File,
        guid: &str,
        content_length: i32,
        last_modified_date_utc: i32,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn competitions_submissions_url(
        &self,
        id: &str,
        content_length: i32,
        last_modified_date_utc: i32,
        file_name: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_create_new(
        &self,
        dataset_new_request: DatasetNewRequest,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_create_version(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        dataset_new_version_request: DatasetNewVersionRequest,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_create_version_by_id(
        &self,
        id: i32,
        dataset_new_version_request: DatasetNewVersionRequest,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_download(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        dataset_version_number: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_download_file(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        file_name: &str,
        dataset_version_number: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_list(
        &self,
        group: &str,
        sort_by: &str,
        size: &str,
        filetype: &str,
        license: &str,
        tagids: &str,
        search: &str,
        user: &str,
        page: usize,
        max_size: i64,
        min_size: i64,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_list_files(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_status(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_upload_file(
        &self,
        file_name: &str,
        content_length: i32,
        last_modified_date_utc: i32,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_view(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn kernel_output(
        &self,
        user_name: &str,
        kernel_slug: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn kernel_pull(&self, user_name: &str, kernel_slug: &str) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn kernel_push(
        &self,
        kernel_push_request: KernelPushRequest,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn kernel_status(
        &self,
        user_name: &str,
        kernel_slug: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn kernels_list(
        &self,
        page: usize,
        page_size: i32,
        search: &str,
        group: &str,
        user: &str,
        language: &str,
        kernel_type: &str,
        output_type: &str,
        sort_by: &str,
        dataset: &str,
        competition: &str,
        parent_kernel: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn metadata_get(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn metadata_post(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        settings: DatasetUpdateSettingsRequest,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn competition_query() {
        // let client = KaggleApiClient::builder().build().unwrap()
    }
}
