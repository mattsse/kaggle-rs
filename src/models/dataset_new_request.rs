use crate::models::{DatasetUploadFile, License};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetNewRequest {
    /// The title of the new dataset
    title: String,
    /// The slug that the dataset should be created with
    slug: Option<String>,
    /// The owner's username
    owner_slug: Option<String>,
    /// The license that should be associated with the dataset
    license_name: Option<String>,
    /// The subtitle to be set on the dataset
    subtitle: Option<String>,
    /// The description to be set on the dataset
    description: Option<String>,
    /// A list of files that should be associated with the dataset
    files: Vec<DatasetUploadFile>,
    /// Whether or not the dataset should be private
    is_private: Option<bool>,
    /// Whether or not a tabular dataset should be converted to csv
    convert_to_csv: Option<bool>,
    /// A list of tag IDs to associated with the dataset
    category_ids: Option<Vec<String>>,
}

impl DatasetNewRequest {
    pub fn builder(title: impl ToString) -> DatasetNewRequestBuilder {
        DatasetNewRequestBuilder::new(title)
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn slug(&self) -> Option<&String> {
        self.slug.as_ref()
    }

    pub fn owner_slug(&self) -> Option<&String> {
        self.owner_slug.as_ref()
    }

    pub fn license_name(&self) -> Option<&String> {
        self.license_name.as_ref()
    }

    pub fn subtitle(&self) -> Option<&String> {
        self.subtitle.as_ref()
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn files(&self) -> &Vec<DatasetUploadFile> {
        &self.files
    }

    pub fn is_private(&self) -> Option<&bool> {
        self.is_private.as_ref()
    }

    pub fn convert_to_csv(&self) -> Option<&bool> {
        self.convert_to_csv.as_ref()
    }

    pub fn category_ids(&self) -> Option<&Vec<String>> {
        self.category_ids.as_ref()
    }
}

pub struct DatasetNewRequestBuilder {
    /// The title of the new dataset
    title: String,
    /// The slug that the dataset should be created with
    slug: Option<String>,
    /// The owner's username
    owner_slug: Option<String>,
    /// The license that should be associated with the dataset
    license_name: Option<String>,
    /// The subtitle to be set on the dataset
    subtitle: Option<String>,
    /// The description to be set on the dataset
    description: Option<String>,
    /// A list of files that should be associated with the dataset
    files: Vec<DatasetUploadFile>,
    /// Whether or not the dataset should be private
    is_private: Option<bool>,
    /// Whether or not a tabular dataset should be converted to csv
    convert_to_csv: Option<bool>,
    /// A list of tag IDs to associated with the dataset
    category_ids: Option<Vec<String>>,
}

impl DatasetNewRequestBuilder {
    pub fn new(title: impl ToString) -> Self {
        Self {
            title: title.to_string(),
            slug: None,
            owner_slug: None,
            license_name: None,
            subtitle: None,
            description: None,
            files: vec![],
            is_private: None,
            convert_to_csv: None,
            category_ids: None,
        }
    }

    pub fn slug(mut self, slug: impl ToString) -> Self {
        self.slug = Some(slug.to_string());
        self
    }

    pub fn owner_slug(mut self, owner_slug: impl ToString) -> Self {
        self.owner_slug = Some(owner_slug.to_string());
        self
    }

    pub fn license_name(mut self, license_name: impl ToString) -> Self {
        self.license_name = Some(license_name.to_string());
        self
    }

    pub fn license(mut self, license: License) -> Self {
        self.license_name = Some(license.to_string());
        self
    }

    pub fn subtitle(mut self, subtitle: impl ToString) -> Self {
        self.subtitle = Some(subtitle.to_string());
        self
    }

    pub fn description(mut self, description: impl ToString) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn files(mut self, files: Vec<DatasetUploadFile>) -> Self {
        self.files = files;
        self
    }

    pub fn file(mut self, file: DatasetUploadFile) -> Self {
        self.files.push(file);
        self
    }

    pub fn private(mut self, private: bool) -> Self {
        self.is_private = Some(private);
        self
    }

    pub fn convert_to_csv(mut self, convert_to_csv: bool) -> Self {
        self.convert_to_csv = Some(convert_to_csv);
        self
    }

    pub fn category_ids(mut self, category_ids: Vec<String>) -> Self {
        self.category_ids = Some(category_ids);
        self
    }

    pub fn category_id(mut self, category_id: impl ToString) -> Self {
        if let Some(ids) = self.category_ids.as_mut() {
            ids.push(category_id.to_string());
            self
        } else {
            self.category_ids(vec![category_id.to_string()])
        }
    }

    pub fn build(self) -> DatasetNewRequest {
        DatasetNewRequest {
            title: self.title,
            slug: self.slug,
            owner_slug: self.owner_slug,
            license_name: self.license_name,
            subtitle: self.subtitle,
            description: self.description,
            files: self.files,
            is_private: self.is_private,
            convert_to_csv: self.convert_to_csv,
            category_ids: self.category_ids,
        }
    }
}
