use std::convert::TryInto;
use std::fmt;
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

use crate::error::KaggleError;
use crate::models::extended::{
    Competition,
    DatasetNewResponse,
    DatasetNewVersionResponse,
    File,
    FileUploadInfo,
    Kernel,
    LeaderboardEntry,
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
use crate::request::{CompetitionsList, KernelsList};
use std::collections::HashMap;
use std::ops::Deref;
use tempdir::TempDir;

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
    download_dir: PathBuf,
}

impl KaggleApiClient {
    const HEADER_API_VERSION: &'static str = "X-Kaggle-ApiVersion";

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
    fn default_headers() -> HeaderMap {
        let headers = HeaderMap::with_capacity(3);
        // TODO do i need this at all?
        headers
    }

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
            base_url: "https://www.kaggle.com/api/v1".parse().unwrap(),
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
    #[inline]
    fn join_url<T: AsRef<str>>(&self, path: T) -> anyhow::Result<Url> {
        Ok(self.base_url.join(path.as_ref())?)
    }

    async fn get<U: IntoUrl>(&self, url: U) -> anyhow::Result<String> {
        Ok(Self::request(self.client.get(url)).await?.text().await?)
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

    /// Write the request's response to the provided output destination.
    async fn download_file(
        req: reqwest::RequestBuilder,
        output: impl AsRef<Path>,
    ) -> anyhow::Result<PathBuf> {
        let mut res = req.send().await?;

        let output = output.as_ref();
        let mut file = tokio::fs::File::create(output).await?;

        while let Some(chunk) = res.chunk().await? {
            file.write_all(&chunk).await?;
        }
        Ok(output.to_path_buf())
    }

    async fn read_metadata_file(path: impl AsRef<Path>) -> anyhow::Result<Metadata> {
        let meta_file = Self::get_dataset_metadata_file(path)?;
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
                    Err(KaggleError::FileNotFound(file))?
                }
            } else {
                Ok(file)
            }
        } else {
            if path.exists() {
                Ok(path)
            } else {
                Err(KaggleError::FileNotFound(path))?
            }
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
        let _ = self.upload_complete(file, &info.create_url).await?;

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

    /// Download competition leaderboard
    pub async fn competition_download_leaderboard<T: AsRef<Path>>(
        &self,
        id: &str,
        target: Option<T>,
    ) -> anyhow::Result<PathBuf> {
        let output = if let Some(target) = target {
            target.as_ref().to_path_buf()
        } else {
            self.download_dir.join(format!("{}-leaderboard.zip", id))
        };

        Ok(Self::download_file(
            self.client
                .get(self.join_url(format!("/competitions/{}/leaderboard/download", id))?),
            output,
        )
        .await?)
    }

    /// View a leaderboard based on a competition name
    pub async fn competition_view_leaderboard(
        &self,
        id: &str,
    ) -> anyhow::Result<Vec<LeaderboardEntry>> {
        Ok(Self::request_json(
            self.client
                .get(self.join_url(format!("/competitions/{}/leaderboard/view", id))?),
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
                .get(self.join_url(format!("/competitions/data/download/{}/{}", id, file_name))?),
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
                .get(self.join_url(format!(" /competitions/data/download-all/{}", id))?),
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

        let meta_data: Metadata = Self::read_metadata_file(folder).await?;

        let (owner_slug, dataset_slug) = meta_data
            .get_user_and_dataset_slug()
            .map(|(s1, s2)| (s1.to_string(), s2.to_string()))?;

        // validate
        if dataset_slug == "INSERT_SLUG_HERE" {
            Err(KaggleError::Metadata {
                msg: "Default slug detected, please change values before uploading".to_string(),
            })?
        }
        if meta_data.title == "INSERT_SLUG_HERE" {
            Err(KaggleError::Metadata {
                msg: "Default title detected, please change values before uploading".to_string(),
            })?
        }
        if meta_data.licenses.len() != 1 {
            Err(KaggleError::Metadata {
                msg: "Please specify exactly one license".to_string(),
            })?
        }
        if dataset_slug.len() < 6 || dataset_slug.len() > 50 {
            Err(KaggleError::Metadata {
                msg: "The dataset slug must be between 6 and 50 characters".to_string(),
            })?
        }
        if meta_data.title.len() < 6 || meta_data.title.len() > 50 {
            Err(KaggleError::Metadata {
                msg: "The dataset title must be between 6 and 50 characters".to_string(),
            })?
        }
        let _ = meta_data.validate_resource(folder)?;

        let mut request = DatasetNewRequest::builder(meta_data.title);
        if let Some(subtitle) = &meta_data.subtitle {
            if subtitle.len() < 20 || subtitle.len() > 80 {
                Err(KaggleError::Metadata {
                    msg: "Subtitle length must be between 20 and 80 characters".to_string(),
                })?
            }
            request = request.subtitle(subtitle);
        }

        let files = self
            .upload_files(folder, &meta_data.resources, archive_mode)
            .await?;

        let request = request
            .slug(dataset_slug)
            .owner_slug(owner_slug)
            .license_name(meta_data.licenses[0].to_string())
            .description(meta_data.description)
            .private(!public)
            .convert_to_csv(convert_to_csv)
            .category_ids(meta_data.keywords)
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
            .post_json("/datasets/create/new", Some(&new_dataset))
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
        let meta_data = Self::read_metadata_file(folder).await?;
        let _ = meta_data.validate_resource(folder)?;

        let mut req = DatasetNewVersionRequest::new(version_notes.to_string());

        let (owner_slug, dataset_slug) = meta_data
            .get_user_and_dataset_slug()
            .map(|(s1, s2)| (s1.to_string(), s2.to_string()))?;

        if let Some(subtitle) = meta_data.subtitle {
            if subtitle.len() < 20 || subtitle.len() > 80 {
                Err(KaggleError::Metadata {
                    msg: "Subtitle length must be between 20 and 80 characters".to_string(),
                })?
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
            if meta_data.id == format!("{}/INSERT_SLUG_HERE", self.credentials.user_name) {
                Err(KaggleError::Metadata {
                    msg: "Default slug detected, please change values before uploading".to_string(),
                })?
            }
            Ok(self
                .datasets_create_version(Some(&owner_slug), &dataset_slug, &req)
                .await?)
        }
    }

    /// Create a new dataset version
    pub async fn datasets_create_version(
        &self,
        owner_slug: Option<&str>,
        dataset_slug: &str,
        dataset_req: &DatasetNewVersionRequest,
    ) -> anyhow::Result<DatasetNewVersionResponse> {
        let owner_slug = owner_slug.unwrap_or_else(|| self.credentials.user_name.as_str());

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

    /// List dataset files
    pub async fn datasets_list_files(
        &self,
        owner_slug: Option<&str>,
        dataset_slug: &str,
    ) -> anyhow::Result<ListFilesResult> {
        let owner_slug = owner_slug.unwrap_or_else(|| self.credentials.user_name.as_str());
        Ok(Self::request_json(
            self.client
                .get(self.join_url(format!("/datasets/list/{}/{}", owner_slug, dataset_slug))?),
        )
        .await?)
    }

    pub async fn datasets_status(
        &self,
        _owner_slug: &str,
        _dataset_slug: &str,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
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
                    "/datasets/upload/file/{}/{}",
                    content_length,
                    last_modified_date_utc.as_secs()
                ))?)
                .multipart(form),
        )
        .await?)
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

    /// Pull the latest code from a kernel
    pub async fn kernel_pull(
        &self,
        user_name: Option<&str>,
        kernel_slug: &str,
    ) -> anyhow::Result<serde_json::Value> {
        let user_name = user_name.unwrap_or_else(|| self.credentials.user_name.as_str());
        Ok(Self::request_json(self.client.get(self.join_url(format!(
            "/kernels/pull?userName={}&kernelSlug={}",
            user_name, kernel_slug
        ))?))
        .await?)
    }

    /// Pull a kernel, including a metadata file (if metadata is True) and
    /// associated files to a specified path.
    pub async fn kernel_pull_write(
        &self,
        user_name: Option<&str>,
        kernel_slug: &str,
        with_metadata: bool,
        output: impl AsRef<Path>,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!()
    }

    /// Push a new kernel version. Can be used to create a new kernel and update
    /// an existing one.
    pub async fn kernel_push(
        &self,
        _kernel_push_request: KernelPushRequest,
    ) -> anyhow::Result<ApiResp> {
        unimplemented!("Not implemented yet.")
    }

    /// Get the status of a kernel.
    pub async fn kernel_status(
        &self,
        user_name: Option<&str>,
        kernel_slug: &str,
    ) -> anyhow::Result<serde_json::Value> {
        let user_name = user_name.unwrap_or_else(|| self.credentials.user_name.as_str());
        Ok(Self::request_json(self.client.get(self.join_url(format!(
            "/kernels/status?userName={}&kernelSlug={}",
            user_name, kernel_slug
        ))?))
        .await?)
    }

    /// List kernels based on a set of search criteria.
    pub async fn kernels_list(&self, kernel_list: &KernelsList) -> anyhow::Result<Vec<Kernel>> {
        Ok(Self::request_json(
            self.client
                .get(self.join_url("/kernels/list")?)
                .query(kernel_list),
        )
        .await?)
    }

    /// Get the metadata for a dataset
    pub async fn metadata_get(
        &self,
        owner_slug: Option<&str>,
        dataset_slug: &str,
    ) -> anyhow::Result<Metadata> {
        let owner_slug = owner_slug.unwrap_or_else(|| self.credentials.user_name.as_str());
        Ok(Self::request_json(self.client.get(self.join_url(format!(
            " /datasets/metadata/{}/{}",
            owner_slug, dataset_slug
        ))?))
        .await?)
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArchiveMode {
    Tar,
    Zip,
    Skip,
}

impl ArchiveMode {
    /// Create either a tar or zip file of the provided source path
    pub fn make_archive(
        &self,
        _src: impl AsRef<Path>,
        _to: impl AsRef<Path>,
    ) -> anyhow::Result<Option<PathBuf>> {
        // TODO implement
        match self {
            ArchiveMode::Tar => {}
            ArchiveMode::Zip => {}
            ArchiveMode::Skip => {}
        }
        Ok(None)
    }
}

impl Default for ArchiveMode {
    fn default() -> Self {
        ArchiveMode::Skip
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
