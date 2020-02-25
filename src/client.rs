use crate::models::{
    DatasetNewRequest,
    DatasetNewVersionRequest,
    DatasetUpdateSettingsRequest,
    KernelPushRequest,
};
use std::fs::File;
use std::rc::Rc;

pub struct KaggleApiClient {
    client: Rc<reqwest::Client>,
}

pub type BasicAuth = (String, Option<String>);

pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    // TODO: take an oauth2 token source, similar to the go one
}

impl Configuration {
    pub fn new() -> Self {
        Configuration {
            base_path: "https://www.kaggle.com/api/v1".to_string(),
            user_agent: Some("kaggele-rs/1/rust".to_string()),
            basic_auth: None,
            oauth_access_token: None,
            api_key: None,
        }
    }
}

pub struct ApiResp;

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
