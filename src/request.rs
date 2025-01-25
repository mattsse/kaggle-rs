use serde::Serialize;

use crate::query::{
    CompetitionCategory, CompetitionGroup, CompetitionSortBy, DatasetFileType, DatasetGroup,
    DatasetLicenseName, Group, KernelType, Language, OutputType, SortBy,
};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompetitionsList {
    /// Group to filter result to
    #[serde(with = "crate::none_as_empty")]
    group: Option<CompetitionGroup>,
    /// Category to filter result to
    #[serde(with = "crate::none_as_empty")]
    category: Option<CompetitionCategory>,
    /// How to sort the result
    #[serde(with = "crate::none_as_empty")]
    sort_by: Option<CompetitionSortBy>,
    /// The page to return.
    page: usize,
    /// Search term to use (default is empty string)
    #[serde(with = "crate::none_as_empty")]
    search: Option<String>,
}

impl CompetitionsList {
    pub fn new(page: usize) -> Self {
        Self {
            group: None,
            category: None,
            sort_by: None,
            page,
            search: None,
        }
    }

    pub fn group(mut self, group: CompetitionGroup) -> Self {
        self.group = Some(group);
        self
    }

    pub fn category(mut self, category: CompetitionCategory) -> Self {
        self.category = Some(category);
        self
    }

    pub fn sort_by(mut self, sort_by: CompetitionSortBy) -> Self {
        self.sort_by = Some(sort_by);
        self
    }

    pub fn search(mut self, search: impl ToString) -> Self {
        self.search = Some(search.to_string());
        self
    }
}

impl Default for CompetitionsList {
    fn default() -> Self {
        Self::new(1)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KernelsList {
    /// The page to return.
    page: usize,
    /// Results per page, defaults to 20
    page_size: usize,
    /// Filter to this dataset
    #[serde(with = "crate::none_as_empty")]
    dataset: Option<String>,
    /// Filter to this competition
    #[serde(with = "crate::none_as_empty")]
    competition: Option<String>,
    /// Filter to those with specified parent
    #[serde(with = "crate::none_as_empty")]
    parent_kernel: Option<String>,
    /// A custom search string to pass to the list query
    #[serde(with = "crate::none_as_empty")]
    search: Option<String>,
    /// whit kind of kernels to return
    group: Group,
    /// Filter results to a specific user
    #[serde(with = "crate::none_as_empty")]
    user: Option<String>,
    /// The programming language of the kernel
    language: Language,
    /// The type of kernel
    kernel_type: KernelType,
    /// The output type
    output_type: OutputType,
    /// Sort results by this string
    sort_by: SortBy,
}

impl Default for KernelsList {
    fn default() -> Self {
        Self::with_page(1)
    }
}

impl KernelsList {
    pub fn with_page(page: usize) -> Self {
        Self {
            page,
            page_size: 20,
            dataset: None,
            competition: None,
            parent_kernel: None,
            search: None,
            group: Default::default(),
            user: None,
            language: Default::default(),
            kernel_type: Default::default(),
            output_type: Default::default(),
            sort_by: Default::default(),
        }
    }

    pub fn page_size(mut self, page_size: usize) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn mine(mut self, group: Group) -> Self {
        self.group = group;
        self
    }

    pub fn dataset(mut self, dataset: impl ToString) -> Self {
        self.dataset = Some(dataset.to_string());
        self
    }

    pub fn competition(mut self, competition: impl ToString) -> Self {
        self.competition = Some(competition.to_string());
        self
    }

    pub fn parent_kernel(mut self, parent_kernel: impl ToString) -> Self {
        self.parent_kernel = Some(parent_kernel.to_string());
        self
    }

    pub fn search(mut self, search: impl ToString) -> Self {
        self.search = Some(search.to_string());
        self
    }

    pub fn user(mut self, user: impl ToString) -> Self {
        self.user = Some(user.to_string());
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }

    pub fn kernel_type(mut self, kernel_type: KernelType) -> Self {
        self.kernel_type = kernel_type;
        self
    }

    pub fn output_type(mut self, output_type: OutputType) -> Self {
        self.output_type = output_type;
        self
    }

    pub fn sort_by(mut self, sort_by: SortBy) -> Self {
        self.sort_by = sort_by;
        self
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetsList {
    /// Display datasets by a particular group
    group: DatasetGroup,
    /// How to sort the result, see valid_dataset_sort_bys for options
    sort_by: SortBy,
    /// The format, see valid_dataset_file_types for string options
    filetype: DatasetFileType,
    /// Tag identifiers to filter the search
    #[serde(with = "crate::none_as_empty")]
    tagids: Option<String>,
    /// Descriptor for the license
    license: DatasetLicenseName,
    /// Search term to use (default is empty string)
    #[serde(with = "crate::none_as_empty")]
    search: Option<String>,
    /// Display datasets by a specific user or organization
    #[serde(with = "crate::none_as_empty")]
    user: Option<String>,
    /// The page to return.
    page: usize,
    /// The maximum size of the dataset to return
    #[serde(skip_serializing_if = "Option::is_none")]
    max_size: Option<usize>,
    /// The minimum size of the dataset to return
    #[serde(skip_serializing_if = "Option::is_none")]
    min_size: Option<usize>,
}

impl DatasetsList {
    pub fn with_page(page: usize) -> Self {
        Self {
            page,
            sort_by: SortBy::Hotness,
            filetype: DatasetFileType::All,
            tagids: None,
            license: DatasetLicenseName::All,
            search: None,
            user: None,
            max_size: None,
            min_size: None,
            group: DatasetGroup::default(),
        }
    }

    pub fn page(mut self, page: usize) -> Self {
        self.page = page;
        self
    }

    pub fn max_size(mut self, max_size: usize) -> Self {
        self.max_size = Some(max_size);
        self
    }

    pub fn min_size(mut self, min_size: usize) -> Self {
        self.min_size = Some(min_size);
        self
    }

    pub fn file_type(mut self, file_type: DatasetFileType) -> Self {
        self.filetype = file_type;
        self
    }

    pub fn search(mut self, search: impl ToString) -> Self {
        self.search = Some(search.to_string());
        self
    }

    pub fn user(mut self, user: impl ToString) -> Self {
        self.user = Some(user.to_string());
        self.group = DatasetGroup::User;
        self
    }

    pub fn mine(mut self) -> Self {
        self.user = None;
        self.group = DatasetGroup::My;
        self
    }

    pub fn public(mut self) -> Self {
        self.user = None;
        self.group = DatasetGroup::Public;
        self
    }

    pub fn license_name(mut self, license_name: DatasetLicenseName) -> Self {
        self.license = license_name;
        self
    }

    pub fn sort_by(mut self, sort_by: SortBy) -> Self {
        self.sort_by = sort_by;
        self
    }

    pub fn tag_ids(mut self, tag_ids: impl ToString) -> Self {
        self.tagids = Some(tag_ids.to_string());
        self
    }
}

impl Default for DatasetsList {
    fn default() -> Self {
        Self::with_page(1)
    }
}

#[derive(Debug, Clone)]
pub struct KernelPullRequest {
    pub with_metadata: bool,
    pub name: String,
    pub output: Option<PathBuf>,
}

impl KernelPullRequest {
    pub fn new(name: impl ToString) -> Self {
        Self {
            with_metadata: false,
            name: name.to_string(),
            output: None,
        }
    }

    pub fn output(mut self, output: impl AsRef<Path>) -> Self {
        self.output = Some(output.as_ref().to_path_buf());
        self
    }

    pub fn with_metadata(mut self, with_metadata: bool) -> Self {
        self.with_metadata = with_metadata;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ser_empty() {
        #[derive(Serialize)]
        struct Dummy {
            #[serde(with = "crate::none_as_empty")]
            group: Option<CompetitionGroup>,
        }
        let x = Dummy { group: None };
        assert_eq!(r#"{"group":""}"#, serde_json::to_string(&x).unwrap());
    }
}
