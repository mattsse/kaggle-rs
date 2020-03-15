use crate::models::{Collaborator, License};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DatasetUpdateSettingsRequest {
    /// Title of the dataset
    title: Option<String>,
    /// Subtitle of the dataset
    subtitle: Option<String>,
    /// Description of the dataset
    description: Option<String>,
    /// Whether or not the dataset should be private
    #[serde(rename = "isPrivate")]
    is_private: Option<bool>,
    /// A list of licenses that apply to this dataset
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    licenses: Vec<License>,
    /// A list of keywords that apply to this dataset
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    keywords: Vec<String>,
    /// A list of collaborators that may read or edit this dataset
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    collaborators: Vec<Collaborator>,
    /// A list containing metadata for each file in the dataset
    #[serde(skip_serializing_if = "Option::is_none", default)]
    data: Option<serde_json::Value>,
}

impl DatasetUpdateSettingsRequest {
    pub fn with_title(title: impl ToString) -> Self {
        Self {
            title: Some(title.to_string()),
            subtitle: None,
            description: None,
            is_private: None,
            licenses: Default::default(),
            keywords: Default::default(),
            collaborators: Default::default(),
            data: Default::default(),
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
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

    pub fn set_licenses(&mut self, licenses: Vec<License>) {
        self.licenses = licenses;
    }

    pub fn with_licenses(mut self, licenses: Vec<License>) -> DatasetUpdateSettingsRequest {
        self.licenses = licenses;
        self
    }

    pub fn licenses(&self) -> &Vec<License> {
        self.licenses.as_ref()
    }

    pub fn set_keywords(&mut self, keywords: Vec<String>) {
        self.keywords = keywords;
    }

    pub fn with_keywords(mut self, keywords: Vec<String>) -> DatasetUpdateSettingsRequest {
        self.keywords = keywords;
        self
    }

    pub fn keywords(&self) -> &Vec<String> {
        self.keywords.as_ref()
    }

    pub fn set_collaborators(&mut self, collaborators: Vec<Collaborator>) {
        self.collaborators = collaborators;
    }

    pub fn with_collaborators(
        mut self,
        collaborators: Vec<Collaborator>,
    ) -> DatasetUpdateSettingsRequest {
        self.collaborators = collaborators;
        self
    }

    pub fn collaborators(&self) -> &Vec<Collaborator> {
        self.collaborators.as_ref()
    }

    pub fn set_data(&mut self, data: serde_json::Value) {
        self.data = Some(data);
    }

    pub fn with_data(mut self, data: serde_json::Value) -> DatasetUpdateSettingsRequest {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&serde_json::Value> {
        self.data.as_ref()
    }
}
