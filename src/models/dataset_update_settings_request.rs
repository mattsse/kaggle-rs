use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DatasetUpdateSettingsRequest {
    /// Title of the dataset
    title: Option<String>,
    /// Subtitle of the dataset
    subtitle: Option<String>,
    /// Decription of the dataset
    description: Option<String>,
    /// Whether or not the dataset should be private
    #[serde(rename = "isPrivate")]
    is_private: Option<bool>,
    /// A list of licenses that apply to this dataset
    licenses: Option<Vec<Value>>,
    /// A list of keywords that apply to this dataset
    keywords: Option<Vec<String>>,
    /// A list of collaborators that may read or edit this dataset
    collaborators: Option<Vec<Value>>,
    /// A list containing metadata for each file in the dataset
    data: Option<Vec<Value>>,
}

impl DatasetUpdateSettingsRequest {
    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    pub fn with_title(mut self, title: String) -> DatasetUpdateSettingsRequest {
        self.title = Some(title);
        self
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn reset_title(&mut self) {
        self.title = None;
    }

    pub fn set_subtitle(&mut self, subtitle: String) {
        self.subtitle = Some(subtitle);
    }

    pub fn with_subtitle(mut self, subtitle: String) -> DatasetUpdateSettingsRequest {
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

    pub fn with_description(mut self, description: String) -> DatasetUpdateSettingsRequest {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_is_private(&mut self, is_private: bool) {
        self.is_private = Some(is_private);
    }

    pub fn with_is_private(mut self, is_private: bool) -> DatasetUpdateSettingsRequest {
        self.is_private = Some(is_private);
        self
    }

    pub fn is_private(&self) -> Option<&bool> {
        self.is_private.as_ref()
    }

    pub fn reset_is_private(&mut self) {
        self.is_private = None;
    }

    pub fn set_licenses(&mut self, licenses: Vec<Value>) {
        self.licenses = Some(licenses);
    }

    pub fn with_licenses(mut self, licenses: Vec<Value>) -> DatasetUpdateSettingsRequest {
        self.licenses = Some(licenses);
        self
    }

    pub fn licenses(&self) -> Option<&Vec<Value>> {
        self.licenses.as_ref()
    }

    pub fn reset_licenses(&mut self) {
        self.licenses = None;
    }

    pub fn set_keywords(&mut self, keywords: Vec<String>) {
        self.keywords = Some(keywords);
    }

    pub fn with_keywords(mut self, keywords: Vec<String>) -> DatasetUpdateSettingsRequest {
        self.keywords = Some(keywords);
        self
    }

    pub fn keywords(&self) -> Option<&Vec<String>> {
        self.keywords.as_ref()
    }

    pub fn reset_keywords(&mut self) {
        self.keywords = None;
    }

    pub fn set_collaborators(&mut self, collaborators: Vec<Value>) {
        self.collaborators = Some(collaborators);
    }

    pub fn with_collaborators(mut self, collaborators: Vec<Value>) -> DatasetUpdateSettingsRequest {
        self.collaborators = Some(collaborators);
        self
    }

    pub fn collaborators(&self) -> Option<&Vec<Value>> {
        self.collaborators.as_ref()
    }

    pub fn reset_collaborators(&mut self) {
        self.collaborators = None;
    }

    pub fn set_data(&mut self, data: Vec<Value>) {
        self.data = Some(data);
    }

    pub fn with_data(mut self, data: Vec<Value>) -> DatasetUpdateSettingsRequest {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&Vec<Value>> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }
}
