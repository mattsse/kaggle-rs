use crate::models::DatasetColumn;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetUploadFile {
    /// A token referencing a specific file upload that can be used across
    /// requests
    #[serde(rename = "token")]
    token: Option<String>,
    /// The file description
    #[serde(rename = "description")]
    description: Option<String>,
    /// A list of dataset column metadata
    #[serde(rename = "columns")]
    columns: Option<Vec<DatasetColumn>>,
}

impl DatasetUploadFile {
    pub fn new() -> DatasetUploadFile {
        DatasetUploadFile {
            token: None,
            description: None,
            columns: None,
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub fn with_token(mut self, token: String) -> DatasetUploadFile {
        self.token = Some(token);
        self
    }

    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }

    pub fn reset_token(&mut self) {
        self.token = None;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn with_description(mut self, description: String) -> DatasetUploadFile {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_columns(&mut self, columns: Vec<DatasetColumn>) {
        self.columns = Some(columns);
    }

    pub fn with_columns(mut self, columns: Vec<DatasetColumn>) -> DatasetUploadFile {
        self.columns = Some(columns);
        self
    }

    pub fn columns(&self) -> Option<&Vec<DatasetColumn>> {
        self.columns.as_ref()
    }

    pub fn reset_columns(&mut self) {
        self.columns = None;
    }
}
