use crate::query::{Language, PushKernelType};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Competition {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub tags: Vec<Tag>,
    pub description: String,
    pub id: i64,
    pub title: String,
    pub url: String,
    #[serde(with = "crate::models::extended::date_serializer")]
    pub deadline: NaiveDateTime,
    pub category: String,
    pub reward: String,
    pub organization_name: Option<String>,
    pub organization_ref: Option<String>,
    pub kernel_count: i64,
    pub team_count: i64,
    pub user_has_entered: bool,
    pub user_rank: Option<i64>,
    #[serde(with = "crate::models::extended::date_serializer_opt")]
    pub merger_deadline: Option<NaiveDateTime>,
    #[serde(with = "crate::models::extended::date_serializer_opt")]
    pub new_entrant_deadline: Option<NaiveDateTime>,
    #[serde(with = "crate::models::extended::date_serializer")]
    pub enabled_date: NaiveDateTime,
    pub max_daily_submissions: i64,
    pub max_team_size: Option<i64>,
    pub evaluation_metric: String,
    pub awards_points: bool,
    pub is_kernels_submissions_only: bool,
    pub submissions_disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitResult {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dataset {
    pub id: i64,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub subtitle: String,
    pub tags: Vec<Tag>,
    pub creator_name: String,
    pub creator_url: String,
    pub total_bytes: i64,
    pub url: String,
    #[serde(with = "crate::models::extended::date_serializer")]
    pub last_updated: NaiveDateTime,
    pub download_count: i64,
    pub is_private: bool,
    pub is_reviewed: bool,
    pub is_featured: bool,
    pub license_name: String,
    pub description: Option<String>,
    pub owner_name: String,
    pub owner_ref: String,
    pub kernel_count: i64,
    pub title: String,
    pub topic_count: i64,
    pub view_count: i64,
    pub vote_count: i64,
    pub current_version_number: i64,
    pub files: Vec<::serde_json::Value>,
    pub versions: Vec<::serde_json::Value>,
    pub usability_rating: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub competition_count: i64,
    pub dataset_count: i64,
    pub description: Option<String>,
    pub full_path: String,
    pub is_automatic: bool,
    pub name: String,
    pub script_count: i64,
    pub total_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetVersion {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUploadInfo {
    pub token: String,
    pub create_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetNewVersionResponse {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetNewResponse {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilesResult {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kernel {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelPullResponse {
    pub blob: KernelBlob,
    pub metadata: Metadata,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl KernelPullResponse {
    pub fn code_file_extension(&self) -> Option<&'static str> {
        self.blob.kernel_type.file_extension(&self.blob.language)
    }

    pub fn code_file_name(&self) -> Option<String> {
        self.blob
            .kernel_type
            .file_extension(&self.blob.language)
            .map(|ext| format!("{}{}", self.blob.slug, ext))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelBlob {
    #[serde(rename = "kernelType")]
    pub kernel_type: PushKernelType,
    pub language: Language,
    pub slug: String,
    pub source: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

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

mod date_serializer {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    fn time_to_json(t: NaiveDateTime) -> String {
        DateTime::<Utc>::from_utc(t, Utc).to_rfc3339()
    }

    pub fn serialize<S: Serializer>(
        time: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        time_to_json(time.clone()).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        Ok(DateTime::parse_from_rfc3339(&time)
            .map(|d| d.naive_utc())
            .map_err(D::Error::custom)?)
    }
}

mod date_serializer_opt {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    fn time_to_json(t: NaiveDateTime) -> String {
        DateTime::<Utc>::from_utc(t, Utc).to_rfc3339()
    }

    pub fn serialize<S: Serializer>(
        time: &Option<NaiveDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if let Some(time) = time {
            time_to_json(time.clone()).serialize(serializer)
        } else {
            time.serialize(serializer)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<NaiveDateTime>, D::Error> {
        let time: Option<String> = Deserialize::deserialize(deserializer)?;
        if let Some(time) = time {
            Ok(Some(
                DateTime::parse_from_rfc3339(&time)
                    .map(|d| d.naive_utc())
                    .map_err(D::Error::custom)?,
            ))
        } else {
            Ok(None)
        }
    }
}
