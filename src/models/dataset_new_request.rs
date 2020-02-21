#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetNewRequest {
    /// The title of the new dataset
    #[serde(rename = "title")]
    title: String,
    /// The slug that the dataset should be created with
    #[serde(rename = "slug")]
    slug: Option<String>,
    /// The owner's username
    #[serde(rename = "ownerSlug")]
    owner_slug: Option<String>,
    /// The license that should be associated with the dataset
    #[serde(rename = "licenseName")]
    license_name: Option<String>,
    /// The subtitle to be set on the dataset
    #[serde(rename = "subtitle")]
    subtitle: Option<String>,
    /// The description to be set on the dataset
    #[serde(rename = "description")]
    description: Option<String>,
    /// A list of files that should be associated with the dataset
    #[serde(rename = "files")]
    files: Vec<::models::DatasetUploadFile>,
    /// Whether or not the dataset should be private
    #[serde(rename = "isPrivate")]
    is_private: Option<bool>,
    /// Whether or not a tabular dataset should be converted to csv
    #[serde(rename = "convertToCsv")]
    convert_to_csv: Option<bool>,
    /// A list of tag IDs to associated with the dataset
    #[serde(rename = "categoryIds")]
    category_ids: Option<Vec<String>>,
}

impl DatasetNewRequest {
    pub fn new(title: String, files: Vec<::models::DatasetUploadFile>) -> DatasetNewRequest {
        DatasetNewRequest {
            title: title,
            slug: None,
            owner_slug: None,
            license_name: None,
            subtitle: None,
            description: None,
            files: files,
            is_private: None,
            convert_to_csv: None,
            category_ids: None,
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn with_title(mut self, title: String) -> DatasetNewRequest {
        self.title = title;
        self
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn set_slug(&mut self, slug: String) {
        self.slug = Some(slug);
    }

    pub fn with_slug(mut self, slug: String) -> DatasetNewRequest {
        self.slug = Some(slug);
        self
    }

    pub fn slug(&self) -> Option<&String> {
        self.slug.as_ref()
    }

    pub fn reset_slug(&mut self) {
        self.slug = None;
    }

    pub fn set_owner_slug(&mut self, owner_slug: String) {
        self.owner_slug = Some(owner_slug);
    }

    pub fn with_owner_slug(mut self, owner_slug: String) -> DatasetNewRequest {
        self.owner_slug = Some(owner_slug);
        self
    }

    pub fn owner_slug(&self) -> Option<&String> {
        self.owner_slug.as_ref()
    }

    pub fn reset_owner_slug(&mut self) {
        self.owner_slug = None;
    }

    pub fn set_license_name(&mut self, license_name: String) {
        self.license_name = Some(license_name);
    }

    pub fn with_license_name(mut self, license_name: String) -> DatasetNewRequest {
        self.license_name = Some(license_name);
        self
    }

    pub fn license_name(&self) -> Option<&String> {
        self.license_name.as_ref()
    }

    pub fn reset_license_name(&mut self) {
        self.license_name = None;
    }

    pub fn set_subtitle(&mut self, subtitle: String) {
        self.subtitle = Some(subtitle);
    }

    pub fn with_subtitle(mut self, subtitle: String) -> DatasetNewRequest {
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

    pub fn with_description(mut self, description: String) -> DatasetNewRequest {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }

    pub fn set_files(&mut self, files: Vec<::models::DatasetUploadFile>) {
        self.files = files;
    }

    pub fn with_files(mut self, files: Vec<::models::DatasetUploadFile>) -> DatasetNewRequest {
        self.files = files;
        self
    }

    pub fn files(&self) -> &Vec<::models::DatasetUploadFile> {
        &self.files
    }

    pub fn set_is_private(&mut self, is_private: bool) {
        self.is_private = Some(is_private);
    }

    pub fn with_is_private(mut self, is_private: bool) -> DatasetNewRequest {
        self.is_private = Some(is_private);
        self
    }

    pub fn is_private(&self) -> Option<&bool> {
        self.is_private.as_ref()
    }

    pub fn reset_is_private(&mut self) {
        self.is_private = None;
    }

    pub fn set_convert_to_csv(&mut self, convert_to_csv: bool) {
        self.convert_to_csv = Some(convert_to_csv);
    }

    pub fn with_convert_to_csv(mut self, convert_to_csv: bool) -> DatasetNewRequest {
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

    pub fn with_category_ids(mut self, category_ids: Vec<String>) -> DatasetNewRequest {
        self.category_ids = Some(category_ids);
        self
    }

    pub fn category_ids(&self) -> Option<&Vec<String>> {
        self.category_ids.as_ref()
    }

    pub fn reset_category_ids(&mut self) {
        self.category_ids = None;
    }
}
