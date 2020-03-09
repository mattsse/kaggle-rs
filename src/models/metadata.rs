use crate::error::KaggleError;
use crate::models::{DatasetColumn, License};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub subtitle: Option<String>,
    pub description: String,
    pub id: String,
    pub licenses: Vec<License>,
    pub resources: Vec<Resource>,
    pub keywords: Vec<String>,
}

impl Metadata {
    pub fn owner_slug(&self) -> Option<&str> {
        self.id.split('/').next()
    }

    pub fn dataset_slug(&self) -> Option<&str> {
        self.id.split('/').nth(1)
    }

    /// Validate resources is a wrapper to validate the existence of files and
    /// that there are no duplicates for a folder and set of resources.
    pub fn validate_resource(&self, root: impl AsRef<Path>) -> Result<(), KaggleError> {
        let root = root.as_ref();
        let mut unique = HashSet::with_capacity(self.resources.len());
        for resource in &self.resources {
            let file = root.join(&resource.path);
            if !file.exists() {
                return Err(KaggleError::FileNotFound(file));
            }
            if !unique.insert(&resource.path) {
                return Err(KaggleError::Metadata {
                    msg: format!(
                        "path {} was specified more than once in the metadata",
                        resource.path
                    ),
                });
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub path: String,
    pub description: String,
    pub schema: Option<Schema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub fields: Vec<Field>,
}

impl Schema {
    /// Process a column, check for the type, and return the processed column.
    pub fn get_processed_columns(&self) -> Vec<DatasetColumn> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
