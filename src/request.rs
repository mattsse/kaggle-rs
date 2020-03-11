use serde::{Deserialize, Serialize, Serializer};

use crate::query::{
    CompetitionCategory,
    CompetitionGroup,
    CompetitionSortBy,
    Group,
    KernelType,
    Language,
    OutputType,
    SortBy,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompetitionsList {
    /// Group to filter result to
    #[serde(with = "crate::none_as_empty")]
    pub group: Option<CompetitionGroup>,
    /// Category to filter result to
    #[serde(with = "crate::none_as_empty")]
    pub category: Option<CompetitionCategory>,
    /// How to sort the result
    #[serde(with = "crate::none_as_empty")]
    pub sort_by: Option<CompetitionSortBy>,
    /// The page to return.
    pub page: usize,
    /// Search term to use (default is empty string)
    #[serde(with = "crate::none_as_empty")]
    pub search: Option<String>,
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
    pub page: usize,
    /// Results per page, defaults to 20
    pub page_size: usize,
    /// Filter to this dataset
    #[serde(with = "crate::none_as_empty")]
    pub dataset: Option<String>,
    /// Filter to this competition
    #[serde(with = "crate::none_as_empty")]
    pub competition: Option<String>,
    /// Filter to those with specified parent
    #[serde(with = "crate::none_as_empty")]
    pub parent_kernel: Option<String>,
    /// A custom search string to pass to the list query
    #[serde(with = "crate::none_as_empty")]
    pub search: Option<String>,
    /// whit kind of kernels to return
    pub group: Group,
    /// Filter results to a specific user
    #[serde(with = "crate::none_as_empty")]
    pub user: Option<String>,
    /// The programming language of the kernel
    pub language: Language,
    /// The type of kernel
    pub kernel_type: KernelType,
    /// The output type
    pub output_type: OutputType,
    /// Sort results by this string
    pub sort_by: SortBy,
}

impl Default for KernelsList {
    fn default() -> Self {
        Self::with_page(1)
    }
}

impl KernelsList {
    pub fn builder() -> Self {
        Self::default()
    }

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
