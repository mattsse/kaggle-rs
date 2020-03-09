use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

use crate::models::DatasetColumn;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DatasetUploadFile {
    /// A token referencing a specific file upload that can be used across
    /// requests
    token: String,
    /// The file description
    description: Option<String>,
    /// A list of dataset column metadata
    columns: Option<Vec<DatasetColumn>>,
}

impl DatasetUploadFile {
    pub fn new(token: impl ToString) -> Self {
        Self {
            token: token.to_string(),
            description: None,
            columns: None,
        }
    }

    pub fn with_token(mut self, token: String) -> DatasetUploadFile {
        self.token = token;
        self
    }

    pub fn token(&self) -> &str {
        &self.token
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
