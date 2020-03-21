use std::convert::TryInto;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;

use bytes::Bytes;
use futures::stream::{self, Stream, StreamExt, TryStreamExt};
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::{multipart, IntoUrl, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncWriteExt};
use tokio_util::codec;
use walkdir::WalkDir;

use anyhow::{anyhow, Context};

use crate::archive::ArchiveMode;
use crate::error::{ApiError, KaggleError};
use crate::models::extended::{
    Competition,
    Dataset,
    DatasetNewResponse,
    DatasetNewVersionResponse,
    File,
    FileUploadInfo,
    Kernel,
    KernelOutput,
    KernelPullResponse,
    KernelPushResponse,
    LeaderBoard,
    ListFilesResult,
    Submission,
    SubmitResult,
};
use crate::models::metadata::{Metadata, Resource};
use crate::models::{
    DatasetNewRequest,
    DatasetNewVersionRequest,
    DatasetUpdateSettingsRequest,
    DatasetUploadFile,
    KernelPushRequest,
};
use crate::query::{PushKernelType, PushLanguageType};
use crate::request::{CompetitionsList, DatasetsList, KernelPullRequest, KernelsList};
use std::collections::HashMap;
use std::ops::Deref;
use tempdir::TempDir;

use log::debug;

/// Client to interact with the kaggle api.
///
/// # Example
///
/// ```
/// # use kaggle::{KaggleApiClient, Authentication};
/// ```
#[derive(Clone)]
pub struct KaggleApiClient {
    /// The client that executes the http requests
    client: Rc<reqwest::Client>,

    /// Base url to the kaggle api, `https://www.kaggle.com/api/v1`
    base_url: Url,

    /// Basic Auth credentials to authenticate the requests
    credentials: KaggleCredentials,

    /// Default location to store downloads
    download_dir: PathBuf,
}

impl KaggleApiClient {
    const DATASET_METADATA_FILE: &'static str = "dataset-metadata.json";

    const OLD_DATASET_METADATA_FILE: &'static str = "datapackage.json";

    const KERNEL_METADATA_FILE: &'static str = "kernel-metadata.json";

    /// Convenience method to create a [`KaggleApiClientBuilder`]
    #[inline]
    pub fn builder() -> KaggleApiClientBuilder {
        KaggleApiClientBuilder::default()
    }

    /// The directory where downloads are stored.
    pub fn download_dir(&self) -> &PathBuf {
        &self.download_dir
    }
}

#[derive(Debug, Clone)]
pub struct KaggleApiClientBuilder {
    base_url: Url,
    user_agent: Option<String>,
    client: Option<Rc<reqwest::Client>>,
    headers: Option<HeaderMap>,
    auth: Option<Authentication>,
    download_dir: Option<PathBuf>,
}

impl KaggleApiClientBuilder {
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn download_dir<T: Into<PathBuf>>(mut self, download_dir: T) -> Self {
        self.download_dir = Some(download_dir.into());
        self
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        if self.headers.is_none() {
            self.headers = Some(HeaderMap::with_capacity(2));
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
            .unwrap_or_else(Authentication::default)
            .credentials()?;

        let mut headers = self.headers.unwrap_or_else(|| HeaderMap::with_capacity(2));

        let mut header_value = b"Basic ".to_vec();
        {
            // See [`reqwest::Request`]
            let mut encoder =
                base64::write::EncoderWriter::new(&mut header_value, base64::STANDARD);
            write!(encoder, "{}:", &credentials.username)?;
            write!(encoder, "{}", &credentials.key)?;
        }

        headers.insert(header::AUTHORIZATION, header_value.try_into()?);
        if let Some(user_agent) = self.user_agent {
            headers.insert(header::USER_AGENT, user_agent.parse()?);
        } else {
            headers.insert(
                header::USER_AGENT,
                HeaderValue::from_static(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION"),
                )),
            );
        }

        let client = if let Some(client) = self.client {
            client
        } else {
            Rc::new(
                reqwest::Client::builder()
                    .default_headers(headers)
                    .build()?,
            )
        };

        let download_dir = if let Some(path) = self.download_dir {
            path
        } else {
            std::env::current_dir()?
        };

        Ok(KaggleApiClient {
            client,
            base_url: self.base_url,
            credentials,
            download_dir,
        })
    }
}

impl Default for KaggleApiClientBuilder {
    fn default() -> Self {
        Self {
            base_url: "https://www.kaggle.com/api/v1/".parse().unwrap(),
            user_agent: None,
            client: None,
            headers: None,
            auth: None,
            download_dir: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KaggleCredentials {
    username: String,
    key: String,
}

impl KaggleCredentials {
    fn from_env() -> anyhow::Result<Self> {
        let user_name = std::env::var("KAGGLE_USERNAME")
            .context("KAGGLE_USERNAME env variable not present.")?;
        let key = std::env::var("KAGGLE_KEY").context("KAGGLE_KEY env variable not present.")?;
        Ok(KaggleCredentials {
            username: user_name,
            key,
        })
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

/// Used to declare the credentials to use for authentication.
///
/// Default is the kaggle.json config file.
#[derive(Debug, Clone)]
pub enum Authentication {
    /// Get the credentials from `KAGGLE_USERNAME` and `KAGGLE_KEY` env
    /// variables.
    Env,

    /// Where the `kaggle.json` file is stored.
    ///
    /// Default location is `~/.kaggle/kaggle.json` and on windows
    /// `C:\Users\<Windows-username>\.kaggle\kaggle.json`
    ConfigFile { path: Option<PathBuf> },

    /// Use dedicated credentials for authentication.
    Credentials { user_name: String, key: String },
}

impl Authentication {
    /// Use dedicated credentials.
    pub fn with_credentials<S: ToString, T: ToString>(user_name: S, key: T) -> Self {
        Authentication::Credentials {
            user_name: user_name.to_string(),
            key: key.to_string(),
        }
    }

    /// Use credentials from a dedicated location.
    pub fn with_config_file(path: impl AsRef<Path>) -> Self {
        Authentication::ConfigFile {
            path: Some(path.as_ref().to_path_buf()),
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
            Authentication::Credentials { user_name, key } => Ok(KaggleCredentials {
                username: user_name,
                key,
            }),
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
    #[inline]
    fn join_url<T: AsRef<str>>(&self, path: T) -> anyhow::Result<Url> {
        Ok(self.base_url.join(path.as_ref())?)
    }

    /// Determine if a dataset string is valid, meaning it is in the format of
    /// {username}/{identifier-slug}
    pub fn get_user_and_identifier_slug<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<(&'a str, &'a str), KaggleError> {
        let mut split = id.split('/');
        if let Some(user) = split.next() {
            if let Some(ident) = split.next() {
                if split.next().is_none() {
                    return Ok((user, ident));
                }
            } else {
                return Ok((&self.credentials.username, user));
            }
        }
        Err(KaggleError::meta( format!(
                "Invalid identifier string. expected form `{{username}}/{{identifier-slug}}`, but got {}",
                id
            ),
        ))
    }

    async fn post_json<T: DeserializeOwned, U: IntoUrl, B: Serialize + ?Sized>(
        &self,
        url: U,
        body: Option<&B>,
    ) -> anyhow::Result<T> {
        let mut req = self.client.post(url).header(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );
        if let Some(body) = body {
            req = req.json(body);
        }
        Ok(Self::request_json(req).await?)
    }

    async fn get_json<T: DeserializeOwned, U: IntoUrl>(&self, url: U) -> anyhow::Result<T> {
        let url = url.into_url()?;
        debug!("GET: {}", url);
        Ok(Self::request_json(self.client.get(url)).await?)
    }

    async fn request_json<T: DeserializeOwned>(req: reqwest::RequestBuilder) -> anyhow::Result<T> {
        println!("Request: {:?}", req);
        let full = Self::request(req).await?.bytes().await?;
        match serde_json::from_slice::<T>(&full) {
            Ok(resp) => Ok(resp),
            Err(err) => {
                if let Ok(api_err) = serde_json::from_slice::<crate::models::Error>(&full) {
                    Err(KaggleError::Api {
                        err: ApiError::ServerError(api_err),
                    }
                    .into())
                } else {
                    Err(err.into())
                }
            }
        }
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
            Err(KaggleError::Api { err }.into())
        }
    }

    /// Write the request's response to the provided output destination.
    async fn download_file(
        mut res: reqwest::Response,
        output: impl AsRef<Path>,
    ) -> anyhow::Result<PathBuf> {
        let output = output.as_ref();
        let mut file = tokio::fs::File::create(output).await?;

        while let Some(chunk) = res.chunk().await? {
            file.write_all(&chunk).await?;
        }
        Ok(output.to_path_buf())
    }

    async fn read_dataset_metadata_file(path: impl AsRef<Path>) -> anyhow::Result<Metadata> {
        let meta_file = Self::get_dataset_metadata_file(path)?;
        let file = tokio::fs::read(&meta_file).await?;
        Ok(serde_json::from_slice(&file)?)
    }

    async fn read_kernel_metadata_file(path: impl AsRef<Path>) -> anyhow::Result<Metadata> {
        let meta_file = Self::get_kernel_metadata_file(path)?;
        let file = tokio::fs::read(&meta_file).await?;
        Ok(serde_json::from_slice(&file)?)
    }

    fn get_dataset_metadata_file(path: impl AsRef<Path>) -> anyhow::Result<PathBuf> {
        let path = path.as_ref().to_path_buf();
        if path.is_dir() {
            let file = path.join(Self::DATASET_METADATA_FILE);
            if !file.exists() {
                let old = path.join(Self::OLD_DATASET_METADATA_FILE);
                if old.exists() {
                    Ok(old)
                } else {
                    Err(KaggleError::FileNotFound(file).into())
                }
            } else {
                Ok(file)
            }
        } else if path.exists() {
            Ok(path)
        } else {
            Err(KaggleError::FileNotFound(path).into())
        }
    }

    fn get_kernel_metadata_file(path: impl AsRef<Path>) -> anyhow::Result<PathBuf> {
        let path = path.as_ref().to_path_buf();
        if path.is_dir() {
            let file = path.join(Self::KERNEL_METADATA_FILE);
            if file.exists() {
                Ok(file)
            } else {
                Err(KaggleError::FileNotFound(file).into())
            }
        } else if path.exists() {
            Ok(path)
        } else {
            Err(KaggleError::FileNotFound(path).into())
        }
    }

    fn get_file_metadata(file: impl AsRef<Path>) -> anyhow::Result<(u64, Duration)> {
        let file = file.as_ref();
        let meta = file.metadata()?;
        let content_length = meta.len();
        let last_modified = meta
            .modified()
            .unwrap_or_else(|_| std::time::SystemTime::now())
            .elapsed()?;

        Ok((content_length, last_modified))
    }

    /// Upload a single dataset file.
    async fn upload_dataset_file(
        &self,
        file: impl AsRef<Path>,
        file_name: impl AsRef<str>,
        item: Option<&Resource>,
    ) -> anyhow::Result<DatasetUploadFile> {
        let file = file.as_ref();
        let (content_length, last_modified) = Self::get_file_metadata(file)?;
        // get the token first
        let info = self
            .datasets_upload_file(file_name.as_ref(), content_length, last_modified)
            .await?;

        // complete the upload to retrieve a path from the url parameter
        self.upload_complete(file, &info.create_url).await?;

        let mut upload_file = DatasetUploadFile::new(info.token);
        if let Some(item) = item {
            upload_file.set_description(item.description.clone());
            if let Some(schema) = &item.schema {
                upload_file.set_columns(schema.get_processed_columns());
            }
            if let Some(schema) = &item.schema {
                upload_file.set_columns(schema.get_processed_columns());
            }
        }

        Ok(upload_file)
    }

    /// Upload files in a folder.
    async fn upload_files(
        &self,
        folder: impl AsRef<Path>,
        resources: &[Resource],
        dir_mode: ArchiveMode,
    ) -> anyhow::Result<Vec<DatasetUploadFile>> {
        let mut uploads = Vec::with_capacity(resources.len());

        let resource_paths: HashMap<_, _> =
            resources.iter().map(|x| (x.path.as_str(), x)).collect();

        let mut tmp_archive_dir = None;

        let skip = &[
            Self::DATASET_METADATA_FILE,
            Self::OLD_DATASET_METADATA_FILE,
            Self::KERNEL_METADATA_FILE,
        ];

        for entry in WalkDir::new(folder)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let file_name = entry
                .path()
                .file_name()
                .context("File path terminates in `..`")?
                .to_str()
                .context("File name is not valid unicode")?;

            let mut upload = None;

            if entry.path().is_file() {
                if skip.contains(&file_name) {
                    continue;
                }
                upload = Some(entry.path().to_path_buf());
            } else if entry.path().is_dir() {
                if tmp_archive_dir.is_none() {
                    tmp_archive_dir = Some(TempDir::new("kaggle-upload")?);
                }
                let archive_path = tmp_archive_dir.as_ref().unwrap().path().join(file_name);
                upload = dir_mode.make_archive(entry.path(), &archive_path)?;
            }

            if let Some(upload) = upload {
                let upload_file = self
                    .upload_dataset_file(
                        upload,
                        file_name,
                        resource_paths.get(file_name).map(Deref::deref),
                    )
                    .await?;
                uploads.push(upload_file);
            }
        }
        if let Some(tmp) = tmp_archive_dir {
            // release all temporary archives
            tmp.close()?;
        }

        Ok(uploads)
    }
}

impl KaggleApiClient {
    /// Returns a list of `Competition'  instances.
    pub async fn competitions_list(
        &self,
        competition: &CompetitionsList,
    ) -> anyhow::Result<Vec<Competition>> {
        Ok(Self::request_json(
            self.client
                .get(self.join_url("competitions/list")?)
                .query(competition),
        )
        .await?)
    }

    /// Download competition leaderboard as zip file, as zip containing a csv of
    /// [`KaggleApiClient::competition_view_leaderboard`].
    ///
    /// If [`output`] is a directory then the destination of the leaderboard zip
    /// file will be `<output>/<id>-leaderboard.zip`.
    ///
    /// If [`output`] is a file then this is the destination of downloaded
    /// leaderboard zip.
    ///
    /// If [`output`] is `None`, then the destination is
    /// `<self.download_dir>/<id>-leaderboard.zip`
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use kaggle::query::CompetitionSortBy;
    /// # use kaggle::request::CompetitionsList;
    /// # use kaggle::KaggleApiClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let kaggle: KaggleApiClient = KaggleApiClient::builder().build()?;
    ///     let resp = kaggle
    ///         .competitions_list(
    ///             &CompetitionsList::default()
    ///                 .sort_by(CompetitionSortBy::RecentlyCreated)
    ///                 .search("health"),
    ///         )
    ///         .await?;
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn competition_download_leaderboard(
        &self,
        id: impl AsRef<str>,
        output: Option<impl AsRef<Path>>,
    ) -> anyhow::Result<PathBuf> {
        let id = id.as_ref();
        let output = if let Some(target) = output {
            let target = target.as_ref();
            if target.is_dir() {
                target.join(format!("{}-leaderboard.zip", id))
            } else {
                target.to_path_buf()
            }
        } else {
            self.download_dir.join(format!("{}-leaderboard.zip", id))
        };

        Ok(Self::download_file(
            self.client
                .get(self.join_url(format!("competitions/{}/leaderboard/download", id))?)
                .send()
                .await?,
            output,
        )
        .await?)
    }

    /// View a leaderboard based on a competition name
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use kaggle::KaggleApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let kaggle: KaggleApiClient = KaggleApiClient::builder().build()?;
    ///     let resp = kaggle
    ///         .competition_view_leaderboard("digit-recognizer")
    ///         .await?;
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn competition_view_leaderboard(
        &self,
        id: impl AsRef<str>,
    ) -> anyhow::Result<LeaderBoard> {
        Ok(Self::request_json(
            self.client
                .get(self.join_url(format!("competitions/{}/leaderboard/view", id.as_ref()))?),
        )
        .await?)
    }

    /// Download a competition data file to a designated location, or use a
    /// default location
    pub async fn competitions_data_download_file<T: AsRef<Path>>(
        &self,
        id: &str,
        file_name: &str,
        target: Option<T>,
    ) -> anyhow::Result<PathBuf> {
        let output = if let Some(target) = target {
            target.as_ref().to_path_buf()
        } else {
            self.download_dir.join(format!("{}.zip", id))
        };

        Ok(Self::download_file(
            self.client
                .get(self.join_url(format!("/competitions/data/download/{}/{}", id, file_name))?)
                .send()
                .await?,
            output,
        )
        .await?)
    }

    /// Downloads all competition files
    pub async fn competitions_data_download_all_files<T: AsRef<Path>>(
        &self,
        id: &str,
        target: Option<T>,
    ) -> anyhow::Result<PathBuf> {
        let output = if let Some(target) = target {
            target.as_ref().to_path_buf()
        } else {
            self.download_dir.join(format!("{}.zip", id))
        };

        Ok(Self::download_file(
            self.client
                .get(self.join_url(format!("/competitions/data/download-all/{}", id))?)
                .send()
                .await?,
            output,
        )
        .await?)
    }

    ///
    pub async fn competitions_data_list_files(&self, id: &str) -> anyhow::Result<Vec<File>> {
        Ok(Self::request_json(
            self.client
                .get(self.join_url(format!("/competitions/data/list/{}", id))?),
        )
        .await?)
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

        Ok(Self::request_json(req).await?)
    }

    /// Submit to competition.
    pub async fn competitions_submissions_submit(
        &self,
        id: impl AsRef<str>,
        blob_file_tokens: impl ToString,
        submission_description: impl ToString,
    ) -> anyhow::Result<SubmitResult> {
        let form = multipart::Form::new()
            .text("blobFileTokens", blob_file_tokens.to_string())
            .text("submissionDescription", submission_description.to_string());

        Ok(Self::request_json(
            self.client
                .post(self.join_url(format!("/competitions/submissions/submit/{}", id.as_ref()))?)
                .multipart(form),
        )
        .await?)
    }

    /// Submit a competition
    pub async fn competition_submit(
        &self,
        file: impl AsRef<Path>,
        competition: impl AsRef<str>,
        message: impl ToString,
    ) -> anyhow::Result<SubmitResult> {
        let competition = competition.as_ref();
        let file = file.as_ref();
        let (content_length, last_modified) = Self::get_file_metadata(&file)?;

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
            .await?;
            url_result
        };

        let token = upload_result
            .as_object()
            .and_then(|x| x.get("token"))
            .and_then(serde_json::Value::as_str)
            .context("Missing upload token")?;

        Ok(self
            .competitions_submissions_submit(competition, token, message)
            .await?)
    }

    pub async fn upload_complete(
        &self,
        file: impl AsRef<Path>,
        url: impl IntoUrl,
    ) -> anyhow::Result<reqwest::Response> {
        let stream = into_bytes_stream(tokio::fs::File::open(file).await?);

        Ok(Self::request(
            self.client
                .put(url)
                .body(reqwest::Body::wrap_stream(stream)),
        )
        .await?)
    }

    /// Upload competition submission file
    pub async fn competitions_submissions_upload(
        &self,
        file: impl AsRef<Path>,
        guid: impl AsRef<str>,
        content_length: u64,
        last_modified_date_utc: Duration,
    ) -> anyhow::Result<serde_json::Value> {
        let stream = into_bytes_stream(tokio::fs::File::open(file).await?);

        let form = multipart::Form::new().part(
            "file",
            multipart::Part::stream(reqwest::Body::wrap_stream(stream)),
        );

        let req = self
            .client
            .post(self.join_url(format!(
                "/competitions/submissions/upload/{}/{}/{}",
                guid.as_ref(),
                content_length,
                last_modified_date_utc.as_secs()
            ))?)
            .multipart(form);

        Ok(Self::request_json(req).await?)
    }

    /// Generate competition submission URL
    pub async fn competitions_submissions_url(
        &self,
        id: impl AsRef<str>,
        content_length: u64,
        last_modified_date_utc: Duration,
        file_name: impl ToString,
    ) -> anyhow::Result<serde_json::Value> {
        let form = multipart::Form::new().text("fileName", file_name.to_string());

        let req = self
            .client
            .post(self.join_url(format!(
                "/competitions/{}/submissions/url/{}/{}",
                id.as_ref(),
                content_length,
                last_modified_date_utc.as_secs()
            ))?)
            .multipart(form);
        Ok(Self::request_json(req).await?)
    }

    /// Create a new dataset, meaning the same as creating a version but with
    /// extra metadata like license and user/owner.
    // TODO convert parameters to struct
    pub async fn dataset_create_new(
        &self,
        folder: impl AsRef<Path>,
        public: bool,
        convert_to_csv: bool,
        archive_mode: ArchiveMode,
    ) -> anyhow::Result<DatasetNewResponse> {
        let folder = folder.as_ref();

        let metadata: Metadata = Self::read_dataset_metadata_file(folder).await?;

        let (owner_slug, dataset_slug) = self
            .get_user_and_identifier_slug(&metadata.id)
            .map(|(s1, s2)| (s1.to_string(), s2.to_string()))?;

        // validate
        if dataset_slug == "INSERT_SLUG_HERE" {
            return Err(KaggleError::meta(
                "Default slug detected, please change values before uploading",
            )
            .into());
        }
        if metadata.title == "INSERT_SLUG_HERE" {
            return Err(KaggleError::meta(
                "Default title detected, please change values before uploading",
            )
            .into());
        }
        if metadata.licenses.len() != 1 {
            return Err(KaggleError::meta("Please specify exactly one license").into());
        }
        if dataset_slug.len() < 6 || dataset_slug.len() > 50 {
            return Err(
                KaggleError::meta("The dataset slug must be between 6 and 50 characters").into(),
            );
        }
        if metadata.title.len() < 6 || metadata.title.len() > 50 {
            return Err(
                KaggleError::meta("The dataset title must be between 6 and 50 characters").into(),
            );
        }
        metadata.validate_resource(folder)?;

        let mut request = DatasetNewRequest::builder(metadata.title);
        if let Some(subtitle) = &metadata.subtitle {
            if subtitle.len() < 20 || subtitle.len() > 80 {
                return Err(KaggleError::meta(
                    "Subtitle length must be between 20 and 80 characters",
                )
                .into());
            }
            request = request.subtitle(subtitle);
        }

        let files = self
            .upload_files(folder, &metadata.resources, archive_mode)
            .await?;

        let request = request
            .slug(dataset_slug)
            .owner_slug(owner_slug)
            .license_name(metadata.licenses[0].to_string())
            .description(metadata.description)
            .private(!public)
            .convert_to_csv(convert_to_csv)
            .category_ids(metadata.keywords)
            .files(files)
            .build();

        Ok(self.datasets_create_new(request).await?)
    }

    /// Create a new dataset.
    pub async fn datasets_create_new(
        &self,
        new_dataset: DatasetNewRequest,
    ) -> anyhow::Result<DatasetNewResponse> {
        Ok(self
            .post_json(self.join_url("/datasets/create/new")?, Some(&new_dataset))
            .await?)
    }

    /// Create a new dataset version
    pub async fn dataset_create_version(
        &self,
        folder: impl AsRef<Path>,
        version_notes: impl ToString,
        convert_to_csv: bool,
        delete_old_versions: bool,
        archive_mode: ArchiveMode,
    ) -> anyhow::Result<DatasetNewVersionResponse> {
        let folder = folder.as_ref();
        let meta_data = Self::read_dataset_metadata_file(folder).await?;
        meta_data.validate_resource(folder)?;

        let mut req = DatasetNewVersionRequest::new(version_notes.to_string());

        if let Some(subtitle) = meta_data.subtitle {
            if subtitle.len() < 20 || subtitle.len() > 80 {
                return Err(KaggleError::Metadata {
                    msg: "Subtitle length must be between 20 and 80 characters".to_string(),
                }
                .into());
            }
            req.set_subtitle(subtitle);
        }

        let files = self
            .upload_files(folder, &meta_data.resources, archive_mode)
            .await?;

        req.set_description(meta_data.description);
        req.set_category_ids(meta_data.keywords);
        req.set_convert_to_csv(convert_to_csv);
        req.set_delete_old_versions(delete_old_versions);
        req.set_files(files);

        if let Some(id_no) = meta_data.id_no {
            Ok(self.datasets_create_version_by_id(id_no, &req).await?)
        } else {
            if meta_data.id == format!("{}/INSERT_SLUG_HERE", self.credentials.username) {
                return Err(KaggleError::Metadata {
                    msg: "Default slug detected, please change values before uploading".to_string(),
                }
                .into());
            }
            Ok(self.datasets_create_version(&meta_data.id, &req).await?)
        }
    }

    /// Create a new dataset version
    pub async fn datasets_create_version(
        &self,
        name: &str,
        dataset_req: &DatasetNewVersionRequest,
    ) -> anyhow::Result<DatasetNewVersionResponse> {
        let (owner_slug, dataset_slug) = self.get_user_and_identifier_slug(name)?;

        Ok(self
            .post_json(
                self.join_url(format!(
                    "/datasets/create/version/{}/{}",
                    owner_slug, dataset_slug
                ))?,
                Some(dataset_req),
            )
            .await?)
    }

    /// Create a new dataset version by id
    pub async fn datasets_create_version_by_id(
        &self,
        id: i32,
        dataset_req: &DatasetNewVersionRequest,
    ) -> anyhow::Result<DatasetNewVersionResponse> {
        Ok(self
            .post_json(
                self.join_url(format!("/datasets/create/version/{}", id))?,
                Some(dataset_req),
            )
            .await?)
    }

    pub async fn dataset_download_all_files(
        &self,
        name: &str,
        path: Option<impl AsRef<Path>>,
        dataset_version_number: Option<&str>,
    ) -> anyhow::Result<PathBuf> {
        let (owner_slug, dataset_slug) = self.get_user_and_identifier_slug(name)?;

        let mut req = self
            .client
            .get(self.join_url(format!(
                "/datasets/download/{}/{}",
                owner_slug, dataset_slug
            ))?)
            .header(header::ACCEPT, HeaderValue::from_static("file"));

        if let Some(version) = dataset_version_number {
            req = req.query(&[("datasetVersionNumber", version)]);
        }

        let resp = Self::request(req).await?;

        let folder = if let Some(path) = path {
            path.as_ref().to_path_buf()
        } else {
            self.download_dir
                .join(format!("datasets/{}/{}", owner_slug, dataset_slug,))
        };
        fs::create_dir_all(&folder)?;

        let outfile =
            Self::download_file(resp, folder.join(format!("{}.zip", dataset_slug))).await?;

        crate::archive::unzip(&outfile)?;

        // TODO add option to keep zip files
        fs::remove_file(outfile)?;

        Ok(folder)
    }

    /// Download a single file for a dataset.
    pub async fn dataset_download_file(
        &self,
        name: &str,
        file_name: &str,
        folder: Option<impl AsRef<Path>>,
        dataset_version_number: Option<&str>,
    ) -> anyhow::Result<PathBuf> {
        let (owner_slug, dataset_slug) = self.get_user_and_identifier_slug(name)?;

        let mut req = self
            .client
            .get(self.join_url(format!(
                "/datasets/download/{}/{}/{}",
                owner_slug, dataset_slug, file_name
            ))?)
            .header(header::ACCEPT, HeaderValue::from_static("file"));

        if let Some(version) = dataset_version_number {
            req = req.query(&[("datasetVersionNumber", version)]);
        }

        let resp = Self::request(req).await?;

        let url = resp
            .url()
            .path_segments()
            .context("redirected to invalid dataset download url")?
            .last()
            .context("no file segment in url download path")?;

        let output = if let Some(folder) = folder {
            folder.as_ref().to_path_buf()
        } else {
            self.download_dir
                .join(format!("datasets/{}/{}", owner_slug, dataset_slug))
        };
        fs::create_dir_all(&output)?;
        let outfile = output.join(url);

        // TODO check if file is already available and is older than the Last-Modified
        // header value
        Ok(Self::download_file(resp, outfile).await?)
    }

    /// List datasets
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use kaggle::request::DatasetsList;
    /// # use kaggle::KaggleApiClient;
    /// # use kaggle::query::SortBy;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let kaggle: KaggleApiClient = KaggleApiClient::builder().build()?;
    ///     let resp = kaggle
    ///         .datasets_list(
    ///             &DatasetsList::default()
    ///                 .sort_by(SortBy::ViewCount)
    ///                 .search("health"),
    ///         )
    ///         .await?;
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn datasets_list(&self, list: &DatasetsList) -> anyhow::Result<Vec<Dataset>> {
        Ok(
            Self::request_json(self.client.get(self.join_url("datasets/list")?).query(list))
                .await?,
        )
    }

    /// List all files for a dataset.
    ///
    /// If the [`name`] is not a combination of
    /// `<user-name-slug>/<dataset-name-slug>` but only a single slug, the
    /// client request to list all the files for the authorized user's dataset
    /// with that name `<client-auth-username>/<name>`.
    ///
    /// # Example
    ///
    /// List all files for a dataset provided by another user.
    ///
    /// ```no_run
    /// # use kaggle::KaggleApiClient;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let kaggle: KaggleApiClient = KaggleApiClient::builder().build()?;
    ///     let resp = kaggle
    ///         .datasets_list_files("allen-institute-for-ai/CORD-19-research-challenge")
    ///         .await?;
    /// #     Ok(())
    /// # }
    /// ```
    /// # Example
    ///
    /// List all files for your own dataset.
    ///
    /// ```no_run
    /// # use kaggle::KaggleApiClient;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let kaggle: KaggleApiClient = KaggleApiClient::builder().build()?;
    ///     let resp = kaggle
    ///         .datasets_list_files("my-awesome-dataset")
    ///         .await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn datasets_list_files(
        &self,
        name: impl AsRef<str>,
    ) -> anyhow::Result<ListFilesResult> {
        let (owner_slug, dataset_slug) = self.get_user_and_identifier_slug(name.as_ref())?;
        Ok(Self::request_json(
            self.client
                .get(self.join_url(format!("datasets/list/{}/{}", owner_slug, dataset_slug))?),
        )
        .await?)
    }

    /// Get dataset creation status.
    pub async fn datasets_status(&self, name: &str) -> anyhow::Result<serde_json::Value> {
        let (owner_slug, dataset_slug) = self.get_user_and_identifier_slug(name)?;
        Ok(self
            .get_json(self.join_url(format!("datasets/status/{}/{}", owner_slug, dataset_slug))?)
            .await?)
    }

    /// Get URL and token to start uploading a data file.
    pub async fn datasets_upload_file(
        &self,
        file_name: impl ToString,
        content_length: u64,
        last_modified_date_utc: Duration,
    ) -> anyhow::Result<FileUploadInfo> {
        let form = multipart::Form::new().text("fileName", file_name.to_string());

        Ok(Self::request_json(
            self.client
                .post(self.join_url(format!(
                    "datasets/upload/file/{}/{}",
                    content_length,
                    last_modified_date_utc.as_secs()
                ))?)
                .multipart(form),
        )
        .await?)
    }

    /// Show details about a dataset.
    pub async fn datasets_view(&self, name: impl AsRef<str>) -> anyhow::Result<Dataset> {
        let (owner_slug, dataset_slug) = self.get_user_and_identifier_slug(name.as_ref())?;
        Ok(self
            .get_json(self.join_url(format!("datasets/view/{}/{}", owner_slug, dataset_slug))?)
            .await?)
    }

    /// Retrieve output for a specified kernel.
    pub async fn kernels_output(
        &self,
        name: impl AsRef<str>,
        path: Option<impl AsRef<Path>>,
    ) -> anyhow::Result<Vec<PathBuf>> {
        let name = name.as_ref();
        let (owner_slug, kernel_slug) = self.get_user_and_identifier_slug(name)?;

        let folder = if let Some(path) = path {
            path.as_ref().to_path_buf()
        } else {
            self.download_dir
                .join(format!("datasets/{}/{}/output", owner_slug, kernel_slug,))
        };
        fs::create_dir_all(&folder)?;

        let resp = self.kernel_output(name).await?;

        let mut outfiles = Vec::with_capacity(resp.files.len());

        let mut outstream = stream::iter(resp.files.into_iter().map(|file| async {
            let outfile = folder.join(file.file_name);
            let content = file.url.content;
            tokio::fs::write(&outfile, content).await?;
            Ok::<_, std::io::Error>(outfile)
        }))
        .buffer_unordered(3);

        while let Some(f) = outstream.next().await {
            outfiles.push(f?);
        }

        if let Some(log) = resp.log {
            let outfile = folder.join(format!("{}.log", kernel_slug));
            tokio::fs::write(&outfile, log).await?;
            outfiles.push(outfile);
        }
        Ok(outfiles)
    }

    /// RDownload the latest output from a kernel
    pub async fn kernel_output(&self, name: &str) -> anyhow::Result<KernelOutput> {
        let (owner_slug, kernel_slug) = self.get_user_and_identifier_slug(name)?;

        if kernel_slug.len() < 5 {
            return Err(KaggleError::meta(format!(
                "Kernel slug `{}` must be at least five characters.",
                kernel_slug
            ))
            .into());
        }

        Ok(self
            .get_json(self.join_url(format!(
                "kernels/output?userName={}&kernelSlug={}",
                owner_slug, kernel_slug
            ))?)
            .await?)
    }

    /// Pull the latest code from a kernel.
    pub async fn kernel_pull(&self, name: &str) -> anyhow::Result<KernelPullResponse> {
        let (owner_slug, kernel_slug) = self.get_user_and_identifier_slug(name)?;
        Ok(self
            .get_json(self.join_url(format!(
                "kernels/pull?userName={}&kernelSlug={}",
                owner_slug, kernel_slug
            ))?)
            .await?)
    }

    /// Pull a kernel, including a metadata file (if metadata is True) and
    /// associated files to a specified path.
    pub async fn kernels_pull(
        &self,
        pull: KernelPullRequest,
    ) -> anyhow::Result<(PathBuf, Option<PathBuf>)> {
        let (owner_slug, kernel_slug) = self.get_user_and_identifier_slug(&pull.name)?;

        let resp = self.kernel_pull(&pull.name).await?;

        let folder = pull.output.unwrap_or_else(|| {
            self.download_dir
                .join(format!("kernels/{}/{}", owner_slug, kernel_slug))
        });
        fs::create_dir_all(&folder)?;

        let metadata_path = folder.join(Self::KERNEL_METADATA_FILE);

        let file_name = if metadata_path.exists() {
            let existing_meta = Self::read_kernel_metadata_file(&metadata_path).await?;
            if Some("INSERT_CODE_FILE_PATH_HERE") == existing_meta.code_file.as_deref() {
                None
            } else {
                existing_meta.code_file
            }
        } else {
            resp.code_file_name()
        }
        .unwrap_or_else(|| "script.py".to_string());

        let output = folder.join(file_name);

        tokio::fs::write(&output, resp.blob.source).await?;

        if pull.with_metadata {
            tokio::fs::write(
                &metadata_path,
                serde_json::to_string_pretty(&resp.metadata)?,
            )
            .await?;

            Ok((output, Some(metadata_path)))
        } else {
            Ok((output, None))
        }
    }

    /// read the metadata file and kernel files from a notebook, validate both,
    /// and use Kernel API to push to Kaggle if all is valid.
    pub async fn kernels_push(
        &self,
        folder: impl AsRef<Path>,
    ) -> anyhow::Result<KernelPushResponse> {
        let folder = folder.as_ref();
        let metadata = Self::read_kernel_metadata_file(folder).await?;

        if metadata.title.len() < 5 {
            return Err(KaggleError::meta("Title must be at least five characters").into());
        }

        metadata.is_dataset_sources_valid()?;
        metadata.is_kernel_sources_valid()?;

        let code_path = metadata
            .code_file
            .ok_or_else(|| KaggleError::meta("A source file must be specified in the metadata"))?;

        let code_file = folder.join(code_path);
        if !code_file.is_file() && !code_file.exists() {
            return Err(KaggleError::meta(format!(
                "Source file not found:{}",
                code_file.display()
            ))
            .into());
        }

        let (_owner_slug, kernel_slug) = self
            .get_user_and_identifier_slug(&metadata.id)
            .map(|(s1, s2)| (s1.to_string(), s2.to_string()))?;

        if kernel_slug.to_lowercase() != slug::slugify(&metadata.title) {
            return Err(
                KaggleError::meta("kernel title does not resolve to the specified id").into(),
            );
        }

        let script_body = tokio::fs::read(&code_file).await?;

        let text = if Some(PushKernelType::Notebook) == metadata.kernel_type {
            let mut json_body = serde_json::from_slice::<serde_json::Value>(&script_body)?;

            // clean outputs
            let obj = json_body
                .as_object_mut()
                .context("Expected json object in code file")?;
            if let Some(cells) = obj.get_mut("cells").and_then(|x| x.as_array_mut()) {
                for cell in cells {
                    if let Some(cell_obj) = cell.as_object_mut() {
                        if cell_obj.contains_key("outputs")
                            && Some("code") == cell_obj.get("cell_type").and_then(|x| x.as_str())
                        {
                            cell_obj
                                .insert("outputs".to_string(), serde_json::Value::Array(vec![]));
                        }
                    }
                }
            }
            serde_json::to_string(&json_body)?
        } else {
            String::from_utf8_lossy(&script_body).to_string()
        };

        let language = if Some(PushKernelType::Notebook) == metadata.kernel_type
            && Some(PushLanguageType::Rmarkdown) == metadata.language
        {
            Some(PushLanguageType::R)
        } else {
            metadata.language
        };

        let mut req = KernelPushRequest::new(text)
            .with_new_title(metadata.title)
            .with_slug(metadata.id)
            .with_dataset_data_sources(metadata.dataset_sources)
            .with_competition_data_sources(metadata.competition_sources)
            .with_kernel_data_sources(metadata.kernel_sources)
            .with_category_ids(metadata.keywords);

        if let Some(id_no) = metadata.id_no {
            req.set_id(id_no);
        }
        if let Some(language) = language {
            req.set_language(language);
        }
        if let Some(kernel) = metadata.kernel_type {
            req.set_kernel_type(kernel);
        }
        if let Some(enable_gpu) = metadata.enable_gpu {
            req.set_enable_gpu(enable_gpu);
        }
        if let Some(enable_internet) = metadata.enable_internet {
            req.set_enable_internet(enable_internet);
        }
        if let Some(is_private) = metadata.is_private {
            req.set_is_private(is_private);
        }

        Ok(self.kernel_push(&req).await?)
    }

    /// Push a new kernel version. Can be used to create a new kernel and update
    /// an existing one.
    pub async fn kernel_push(
        &self,
        kernel_push_request: &KernelPushRequest,
    ) -> anyhow::Result<KernelPushResponse> {
        Ok(self
            .post_json(self.join_url("/kernels/push")?, Some(kernel_push_request))
            .await?)
    }

    /// Get the status of a kernel.
    pub async fn kernel_status(&self, name: &str) -> anyhow::Result<serde_json::Value> {
        let (owner_slug, kernel_slug) = self.get_user_and_identifier_slug(name)?;
        Ok(Self::request_json(self.client.get(self.join_url(format!(
            "kernels/status?userName={}&kernelSlug={}",
            owner_slug, kernel_slug
        ))?))
        .await?)
    }

    /// List kernels based on a set of search criteria.
    pub async fn kernels_list(&self, kernel_list: &KernelsList) -> anyhow::Result<Vec<Kernel>> {
        Ok(Self::request_json(
            self.client
                .get(self.join_url("kernels/list")?)
                .query(kernel_list),
        )
        .await?)
    }

    /// Get the metadata for a dataset.
    pub async fn metadata_get(&self, name: &str) -> anyhow::Result<Metadata> {
        let (owner_slug, dataset_slug) = self.get_user_and_identifier_slug(name)?;
        Ok(Self::request_json(
            self.client
                .get(self.join_url(format!("datasets/metadata/{}/{}", owner_slug, dataset_slug))?),
        )
        .await?)
    }

    /// Update the metadata for a dataset
    pub async fn dataset_metadata_update(
        &self,
        name: &str,
        path: Option<impl AsRef<Path>>,
    ) -> anyhow::Result<serde_json::Value> {
        let metadata = if let Some(path) = path {
            Self::read_dataset_metadata_file(path).await?
        } else {
            let (owner_slug, dataset_slug) = self.get_user_and_identifier_slug(name)?;
            Self::read_dataset_metadata_file(
                self.download_dir
                    .join(format!("datasets/{}/{}", owner_slug, dataset_slug)),
            )
            .await?
        };

        let settings = metadata.into();
        Ok(self.metadata_post(name, &settings).await?)
    }

    pub async fn metadata_post(
        &self,
        name: &str,
        settings: &DatasetUpdateSettingsRequest,
    ) -> anyhow::Result<serde_json::Value> {
        let (owner_slug, dataset_slug) = self.get_user_and_identifier_slug(name)?;

        Ok(self
            .post_json(
                self.join_url(format!("datasets/metadata/{}/{}", owner_slug, dataset_slug))?,
                Some(settings),
            )
            .await?)
    }
}

fn into_bytes_stream<R>(r: R) -> impl Stream<Item = tokio::io::Result<Bytes>>
where
    R: AsyncRead,
{
    codec::FramedRead::new(r, codec::BytesCodec::new()).map_ok(|bytes| bytes.freeze())
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
