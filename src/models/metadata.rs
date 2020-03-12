use crate::error::KaggleError;
use crate::models::{DatasetColumn, License};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub subtitle: Option<String>,
    pub description: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id_no: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub licenses: Vec<License>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub resources: Vec<Resource>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub keywords: Vec<String>,
}

impl Metadata {
    pub fn owner_slug(&self) -> Option<&str> {
        self.id.split('/').next()
    }

    pub fn dataset_slug(&self) -> Option<&str> {
        self.id.split('/').nth(1)
    }

    /// determine if a dataset string is valid, meaning it is in the format of
    /// {username}/{dataset-slug}
    pub fn get_user_and_dataset_slug(&self) -> Result<(&str, &str), KaggleError> {
        let mut split = self.id.split('/');
        if let Some(user) = split.next() {
            if let Some(dataset) = split.next() {
                if split.next().is_none() {
                    return Ok((user, dataset));
                }
            }
        }
        Err(KaggleError::Metadata {
            msg: format!(
                "Invalid dataset string. expected form `{{username}}/{{dataset-slug}}`, but got {}",
                self.id
            ),
        })
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub schema: Option<Schema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub fields: Vec<Field>,
}

impl Schema {
    /// Process a column, check for the type, and return the processed column.
    pub fn get_processed_columns(&self) -> Vec<DatasetColumn> {
        let mut columns = Vec::with_capacity(self.fields.len());

        let str_types = &[
            "string",
            "date",
            "time",
            "yearmonth",
            "duration",
            "geopoint",
            "geojson",
        ];

        for field in &self.fields {
            let mut col = DatasetColumn::new(field.name.clone());
            if let Some(desc) = &field.description {
                col.set_description(desc.clone());
            }
            if let Some(ty) = &field.type_field {
                let ty = ty.to_lowercase();

                if str_types.contains(&ty.as_str()) {
                    col.set_type("string".to_string());
                } else if ty == "numeric" || ty == "number" || ty == "year" {
                    col.set_type("numeric".to_string());
                } else if ty == "boolean" {
                    col.set_type("boolean".to_string());
                } else if ty == "datetime" {
                    col.set_type("datetime".to_string());
                } else {
                    // Possibly extended data type - not going to try to track those here. Will set
                    // the type and let the server handle it.
                    col.set_type(ty.clone());
                }
                col.set_original_type(ty);
            }
            columns.push(col);
        }

        columns
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub type_field: Option<String>,
}
