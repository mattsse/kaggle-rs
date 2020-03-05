use crate::query::{CompetitionCategory, CompetitionGroup, CompetitionSortBy};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Serialize)]
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
            page,
            search: None,
            ..Default::default()
        }
    }
}

impl Default for CompetitionsList {
    fn default() -> Self {
        Self::new(1)
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
