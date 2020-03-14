use crate::query::{PushKernelType, PushLanguageType};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KernelPushRequest {
    /// The kernel's ID number. One of `id` and `slug` are required. If both are
    /// specified, `id` will be preferred
    id: Option<i32>,
    /// The full slug of the kernel to push to, in the format
    /// `USERNAME/KERNEL-SLUG`. The kernel slug must be the title lowercased
    /// with dashes (`-`) replacing spaces. One of `id` and `slug` are required.
    /// If both are specified, `id` will be preferred
    slug: Option<String>,
    /// The title to be set on the kernel
    new_title: Option<String>,
    /// The kernel's source code
    text: String,
    /// The language that the kernel is written in
    #[serde(with = "crate::none_as_empty")]
    language: Option<PushLanguageType>,
    /// The type of kernel. Cannot be changed once the kernel has been created
    #[serde(with = "crate::none_as_empty")]
    kernel_type: Option<PushKernelType>,
    /// Whether or not the kernel should be private
    is_private: Option<bool>,
    /// Whether or not the kernel should run on a GPU
    enable_gpu: Option<bool>,
    /// Whether or not the kernel should be able to access the internet
    enable_internet: Option<bool>,
    /// A list of dataset data sources that the kernel should use. Each dataset
    /// is specified as `USERNAME/DATASET-SLUG`
    dataset_data_sources: Option<Vec<String>>,
    /// A list of competition data sources that the kernel should use
    competition_data_sources: Option<Vec<String>>,
    /// A list of kernel data sources that the kernel should use. Each dataset
    /// is specified as `USERNAME/KERNEL-SLUG`
    kernel_data_sources: Option<Vec<String>>,
    /// A list of tag IDs to associated with the dataset
    category_ids: Option<Vec<String>>,
}

impl KernelPushRequest {
    pub fn new(text: String) -> Self {
        KernelPushRequest {
            id: None,
            slug: None,
            new_title: None,
            text,
            language: None,
            kernel_type: None,
            is_private: None,
            enable_gpu: None,
            enable_internet: None,
            dataset_data_sources: None,
            competition_data_sources: None,
            kernel_data_sources: None,
            category_ids: None,
        }
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: i32) -> KernelPushRequest {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&i32> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_slug(&mut self, slug: String) {
        self.slug = Some(slug);
    }

    pub fn with_slug(mut self, slug: String) -> KernelPushRequest {
        self.slug = Some(slug);
        self
    }

    pub fn slug(&self) -> Option<&String> {
        self.slug.as_ref()
    }

    pub fn reset_slug(&mut self) {
        self.slug = None;
    }

    pub fn set_new_title(&mut self, new_title: String) {
        self.new_title = Some(new_title);
    }

    pub fn with_new_title(mut self, new_title: String) -> KernelPushRequest {
        self.new_title = Some(new_title);
        self
    }

    pub fn new_title(&self) -> Option<&String> {
        self.new_title.as_ref()
    }

    pub fn reset_new_title(&mut self) {
        self.new_title = None;
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn with_text(mut self, text: String) -> KernelPushRequest {
        self.text = text;
        self
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn set_language(&mut self, language: PushLanguageType) {
        self.language = Some(language);
    }

    pub fn with_language(mut self, language: PushLanguageType) -> KernelPushRequest {
        self.language = Some(language);
        self
    }

    pub fn language(&self) -> Option<&PushLanguageType> {
        self.language.as_ref()
    }

    pub fn set_kernel_type(&mut self, kernel_type: PushKernelType) {
        self.kernel_type = Some(kernel_type);
    }

    pub fn with_kernel_type(mut self, kernel_type: PushKernelType) -> KernelPushRequest {
        self.kernel_type = Some(kernel_type);
        self
    }

    pub fn kernel_type(&self) -> Option<&PushKernelType> {
        self.kernel_type.as_ref()
    }

    pub fn set_is_private(&mut self, is_private: bool) {
        self.is_private = Some(is_private);
    }

    pub fn with_is_private(mut self, is_private: bool) -> KernelPushRequest {
        self.is_private = Some(is_private);
        self
    }

    pub fn is_private(&self) -> Option<&bool> {
        self.is_private.as_ref()
    }

    pub fn reset_is_private(&mut self) {
        self.is_private = None;
    }

    pub fn set_enable_gpu(&mut self, enable_gpu: bool) {
        self.enable_gpu = Some(enable_gpu);
    }

    pub fn with_enable_gpu(mut self, enable_gpu: bool) -> KernelPushRequest {
        self.enable_gpu = Some(enable_gpu);
        self
    }

    pub fn enable_gpu(&self) -> Option<&bool> {
        self.enable_gpu.as_ref()
    }

    pub fn reset_enable_gpu(&mut self) {
        self.enable_gpu = None;
    }

    pub fn set_enable_internet(&mut self, enable_internet: bool) {
        self.enable_internet = Some(enable_internet);
    }

    pub fn with_enable_internet(mut self, enable_internet: bool) -> KernelPushRequest {
        self.enable_internet = Some(enable_internet);
        self
    }

    pub fn enable_internet(&self) -> Option<&bool> {
        self.enable_internet.as_ref()
    }

    pub fn reset_enable_internet(&mut self) {
        self.enable_internet = None;
    }

    pub fn set_dataset_data_sources(&mut self, dataset_data_sources: Vec<String>) {
        self.dataset_data_sources = Some(dataset_data_sources);
    }

    pub fn with_dataset_data_sources(
        mut self,
        dataset_data_sources: Vec<String>,
    ) -> KernelPushRequest {
        self.dataset_data_sources = Some(dataset_data_sources);
        self
    }

    pub fn dataset_data_sources(&self) -> Option<&Vec<String>> {
        self.dataset_data_sources.as_ref()
    }

    pub fn reset_dataset_data_sources(&mut self) {
        self.dataset_data_sources = None;
    }

    pub fn set_competition_data_sources(&mut self, competition_data_sources: Vec<String>) {
        self.competition_data_sources = Some(competition_data_sources);
    }

    pub fn with_competition_data_sources(
        mut self,
        competition_data_sources: Vec<String>,
    ) -> KernelPushRequest {
        self.competition_data_sources = Some(competition_data_sources);
        self
    }

    pub fn competition_data_sources(&self) -> Option<&Vec<String>> {
        self.competition_data_sources.as_ref()
    }

    pub fn reset_competition_data_sources(&mut self) {
        self.competition_data_sources = None;
    }

    pub fn set_kernel_data_sources(&mut self, kernel_data_sources: Vec<String>) {
        self.kernel_data_sources = Some(kernel_data_sources);
    }

    pub fn with_kernel_data_sources(
        mut self,
        kernel_data_sources: Vec<String>,
    ) -> KernelPushRequest {
        self.kernel_data_sources = Some(kernel_data_sources);
        self
    }

    pub fn kernel_data_sources(&self) -> Option<&Vec<String>> {
        self.kernel_data_sources.as_ref()
    }

    pub fn reset_kernel_data_sources(&mut self) {
        self.kernel_data_sources = None;
    }

    pub fn set_category_ids(&mut self, category_ids: Vec<String>) {
        self.category_ids = Some(category_ids);
    }

    pub fn with_category_ids(mut self, category_ids: Vec<String>) -> KernelPushRequest {
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
