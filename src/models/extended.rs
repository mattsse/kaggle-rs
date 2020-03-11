use serde::{Deserialize, Serialize};

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
pub struct KernelPushResponse {}
