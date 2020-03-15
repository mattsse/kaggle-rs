use crate::error::KaggleError;
use crate::models::{Collaborator, DatasetColumn, DatasetUpdateSettingsRequest, License};
use crate::query::{PushKernelType, PushLanguageType};
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub code_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub language: Option<PushLanguageType>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub kernel_type: Option<PushKernelType>,
    /// Whether or not the dataset should be private
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub enable_gpu: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub enable_internet: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub collaborators: Vec<Collaborator>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub licenses: Vec<License>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub resources: Vec<Resource>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub keywords: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub kernel_sources: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub dataset_sources: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub competition_sources: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub data: Option<serde_json::Value>,
}

impl Metadata {
    pub fn is_valid_kernel_string(s: impl AsRef<str>) -> Result<(), KaggleError> {
        let s = s.as_ref();
        let mut split = s.split('/');
        if let Some(kernel_slug) = split.nth(1) {
            if kernel_slug.len() < 5 {
                Err(KaggleError::meta(format!(
                    "Kernel slug `{}` must be at least five characters.",
                    kernel_slug
                )))
            } else {
                Ok(())
            }
        } else {
            Err(KaggleError::meta(format!(
                "Invalid kernel source identifier. expected form `{{username}}/{{identifier-slug}}`, but got {}",
                s
            ),
            ))
        }
    }

    pub fn owner_slug(&self) -> Option<&str> {
        self.id.split('/').next()
    }

    pub fn dataset_slug(&self) -> Option<&str> {
        self.id.split('/').nth(1)
    }

    pub fn is_dataset_sources_valid(&self) -> Result<(), KaggleError> {
        for s in &self.dataset_sources {
            if s.split('/').count() < 2 {
                return Err(KaggleError::meta(format!(
                    "Invalid dataset source identifier. expected form `{{username}}/{{identifier-slug}}`, but got {}",
                    s
                ),
                ));
            }
        }
        Ok(())
    }

    pub fn is_kernel_sources_valid(&self) -> Result<(), KaggleError> {
        for s in &self.kernel_sources {
            Self::is_valid_kernel_string(s)?;
        }
        Ok(())
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

impl Into<DatasetUpdateSettingsRequest> for Metadata {
    fn into(self) -> DatasetUpdateSettingsRequest {
        let mut settings = DatasetUpdateSettingsRequest::with_title(self.title)
            .with_description(self.description)
            .with_licenses(self.licenses)
            .with_keywords(self.keywords)
            .with_collaborators(self.collaborators);
        if let Some(s) = self.subtitle {
            settings.set_subtitle(s);
        }
        if let Some(p) = self.is_private {
            settings.set_is_private(p);
        }
        if let Some(d) = self.data {
            settings.set_data(d);
        }
        settings
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
