use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competition {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitResult {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetVersion {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUploadInfo {
    pub token: String,
    pub create_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetNewVersionResponse {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetNewResponse {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilesResult {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kernel {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelPushResponse {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelOutput {
    #[serde(default)]
    pub files: Vec<KernelOutputFile>,
    pub log: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelOutputFile {
    #[serde(rename = "fileName")]
    pub file_name: String,
    pub url: DownloadResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResponse {
    pub content: String,
}
