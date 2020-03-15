use serde::{Deserialize, Serialize};

use crate::models::DatasetColumn;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DatasetUploadFile {
    /// A token referencing a specific file upload that can be used across
    /// requests
    token: String,
    /// The file description
    description: Option<String>,
    /// A list of dataset column metadata
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    columns: Vec<DatasetColumn>,
}

impl DatasetUploadFile {
    pub fn new(token: impl ToString) -> Self {
        Self {
            token: token.to_string(),
            ..Default::default()
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
        self.columns = columns;
    }

    pub fn columns(&self) -> &Vec<DatasetColumn> {
        self.columns.as_ref()
    }
}
