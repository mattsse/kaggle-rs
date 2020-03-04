use crate::config::Config;
use crate::models::{
    DatasetNewRequest, DatasetNewVersionRequest, DatasetUpdateSettingsRequest, KernelPushRequest,
};
use anyhow::{anyhow, Context};
use reqwest::header::{self, HeaderMap};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::convert::TryInto;
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

#[derive(Clone)]
pub struct KaggleApiClient {
    client: Rc<reqwest::Client>,
}

impl KaggleApiClient {
    const HEADER_API_VERSION: &'static str = "X-Kaggle-ApiVersion";
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

    // TODO should take an arg how to authenticate
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
            write!(encoder, "{}:", credentials.user_name)?;
            write!(encoder, "{}", credentials.key)?;
        }

        headers.insert(header::AUTHORIZATION, header_value.try_into()?);

        unimplemented!()
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
    pub fn convert_result<'a, T: Deserialize<'a>>(&self, input: &'a str) -> Result<T, String> {
        let result = serde_json::from_str::<T>(input).map_err(|e| {
            format!(
                "convert result failed, reason: {:?}; content: [{:?}]",
                e, input
            )
        })?;
        Ok(result)
    }
}

impl KaggleApiClient {
    pub async fn competition_download_leaderboard(
        &self,
        id: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn competition_view_leaderboard(
        &self,
        id: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn competitions_data_download_file(
        &self,
        id: &str,
        file_name: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn competitions_data_download_files(
        &self,
        id: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn competitions_data_list_files(
        &self,
        id: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn competitions_list(
        &self,
        group: &str,
        category: &str,
        sort_by: &str,
        page: i32,
        search: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn competitions_submissions_list(
        &self,
        id: &str,
        page: i32,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn competitions_submissions_submit(
        &self,
        blob_file_tokens: &str,
        submission_description: &str,
        id: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn competitions_submissions_upload(
        &self,
        file: File,
        guid: &str,
        content_length: i32,
        last_modified_date_utc: i32,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn competitions_submissions_url(
        &self,
        id: &str,
        content_length: i32,
        last_modified_date_utc: i32,
        file_name: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_create_new(
        &self,
        dataset_new_request: DatasetNewRequest,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_create_version(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        dataset_new_version_request: DatasetNewVersionRequest,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_create_version_by_id(
        &self,
        id: i32,
        dataset_new_version_request: DatasetNewVersionRequest,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_download(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        dataset_version_number: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_download_file(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        file_name: &str,
        dataset_version_number: &str,
    ) -> Result<ApiResp, serde_json::Value> {
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
        page: i32,
        max_size: i64,
        min_size: i64,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_list_files(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_status(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_upload_file(
        &self,
        file_name: &str,
        content_length: i32,
        last_modified_date_utc: i32,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn datasets_view(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn kernel_output(
        &self,
        user_name: &str,
        kernel_slug: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn kernel_pull(
        &self,
        user_name: &str,
        kernel_slug: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn kernel_push(
        &self,
        kernel_push_request: KernelPushRequest,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn kernel_status(
        &self,
        user_name: &str,
        kernel_slug: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn kernels_list(
        &self,
        page: i32,
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
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn metadata_get(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
    pub async fn metadata_post(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        settings: DatasetUpdateSettingsRequest,
    ) -> Result<ApiResp, serde_json::Value> {
        unimplemented!("Not implemented yet.")
    }
}
