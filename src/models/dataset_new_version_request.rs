use crate::models::DatasetUploadFile;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetNewVersionRequest {
    /// The version notes for the new dataset version
    #[serde(rename = "versionNotes")]
    version_notes: String,
    /// The subtitle to set on the dataset
    subtitle: Option<String>,
    /// The description to set on the dataset
    description: Option<String>,
    /// A list of files that should be associated with the dataset
    files: Vec<DatasetUploadFile>,
    /// Whether or not a tabular dataset should be converted to csv
    #[serde(rename = "convertToCsv")]
    convert_to_csv: Option<bool>,
    /// A list of tag IDs to associated with the dataset
    #[serde(rename = "categoryIds")]
    category_ids: Option<Vec<String>>,
    /// Whether or not all previous versions of the dataset should be deleted
    /// upon creating the new version
    #[serde(rename = "deleteOldVersions")]
    delete_old_versions: Option<bool>,
}

impl DatasetNewVersionRequest {
    pub fn new(version_notes: String, files: Vec<DatasetUploadFile>) -> DatasetNewVersionRequest {
        DatasetNewVersionRequest {
            version_notes,
            subtitle: None,
            description: None,
            files,
            convert_to_csv: None,
            category_ids: None,
            delete_old_versions: None,
        }
    }

    pub fn set_version_notes(&mut self, version_notes: String) {
        self.version_notes = version_notes;
    }

    pub fn with_version_notes(mut self, version_notes: String) -> DatasetNewVersionRequest {
        self.version_notes = version_notes;
        self
    }

    pub fn version_notes(&self) -> &String {
        &self.version_notes
    }

    pub fn set_subtitle(&mut self, subtitle: String) {
        self.subtitle = Some(subtitle);
    }

    pub fn with_subtitle(mut self, subtitle: String) -> DatasetNewVersionRequest {
        self.subtitle = Some(subtitle);
        self
    }

    pub fn subtitle(&self) -> Option<&String> {
        self.subtitle.as_ref()
    }

    pub fn reset_subtitle(&mut self) {
        self.subtitle = None;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn with_description(mut self, description: String) -> DatasetNewVersionRequest {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_files(&mut self, files: Vec<DatasetUploadFile>) {
        self.files = files;
    }

    pub fn with_files(mut self, files: Vec<DatasetUploadFile>) -> DatasetNewVersionRequest {
        self.files = files;
        self
    }

    pub fn files(&self) -> &Vec<DatasetUploadFile> {
        &self.files
    }

    pub fn set_convert_to_csv(&mut self, convert_to_csv: bool) {
        self.convert_to_csv = Some(convert_to_csv);
    }

    pub fn with_convert_to_csv(mut self, convert_to_csv: bool) -> DatasetNewVersionRequest {
        self.convert_to_csv = Some(convert_to_csv);
        self
    }

    pub fn convert_to_csv(&self) -> Option<&bool> {
        self.convert_to_csv.as_ref()
    }

    pub fn reset_convert_to_csv(&mut self) {
        self.convert_to_csv = None;
    }

    pub fn set_category_ids(&mut self, category_ids: Vec<String>) {
        self.category_ids = Some(category_ids);
    }

    pub fn with_category_ids(mut self, category_ids: Vec<String>) -> DatasetNewVersionRequest {
        self.category_ids = Some(category_ids);
        self
    }

    pub fn category_ids(&self) -> Option<&Vec<String>> {
        self.category_ids.as_ref()
    }

    pub fn reset_category_ids(&mut self) {
        self.category_ids = None;
    }

    pub fn set_delete_old_versions(&mut self, delete_old_versions: bool) {
        self.delete_old_versions = Some(delete_old_versions);
    }

    pub fn with_delete_old_versions(
        mut self,
        delete_old_versions: bool,
    ) -> DatasetNewVersionRequest {
        self.delete_old_versions = Some(delete_old_versions);
        self
    }

    pub fn delete_old_versions(&self) -> Option<&bool> {
        self.delete_old_versions.as_ref()
    }

    pub fn reset_delete_old_versions(&mut self) {
        self.delete_old_versions = None;
    }
}
