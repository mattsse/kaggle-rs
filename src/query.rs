use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PushKernelType {
    Script,
    Notebook,
}

impl PushKernelType {
    pub fn file_extension(&self, language: &Language) -> Option<&'static str> {
        match self {
            PushKernelType::Script => match language {
                Language::Python => Some(".py"),
                Language::R => Some(".r"),
                Language::Sqlite => Some(".sqlite"),
                Language::Julia => Some(".jl"),
                Language::Rmarkdown => Some(".Rmd"),
                _ => None,
            },
            PushKernelType::Notebook => match language {
                Language::Python => Some(".ipynb"),
                Language::R => Some(".irnb"),
                Language::Julia => Some(".ijlnb"),
                _ => None,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PushLanguageType {
    Python,
    R,
    Rmarkdown,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum Language {
    #[default]
    All,
    Python,
    R,
    Sqlite,
    Julia,
    Rmarkdown,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum KernelType {
    #[default]
    All,
    Script,
    Notebook,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum OutputType {
    #[default]
    All,
    Visualization,
    Data,
}

/// How to sort the result
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum SortBy {
    #[default]
    Hotness,
    CommentCount,
    DateCreated,
    DateRun,
    Relevance,
    ScoreAscending,
    ScoreDescending,
    ViewCount,
    VoteCount,
}

/// Competitoins valid types
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum CompetitionGroup {
    #[default]
    General,
    Entered,
    InClass,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum CompetitionCategory {
    #[default]
    All,
    Featured,
    Research,
    Recruitment,
    GettingStarted,
    Masters,
    Playground,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum CompetitionSortBy {
    Grouped,
    Prize,
    EarliestDeadline,
    #[default]
    LatestDeadline,
    NumberOfTeams,
    RecentlyCreated,
}

/// Datasets valid types
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum DatasetFileType {
    #[default]
    All,
    Csv,
    Sqlite,
    Json,
    BigQuery,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum DatasetLicenseName {
    #[default]
    All,
    Cc,
    Gpl,
    Odb,
    Other,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DatasetSortBy {
    Hottest,
    Votes,
    Updated,
    Active,
    Published,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum Group {
    #[default]
    Everyone,
    Profile,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub(crate) enum DatasetGroup {
    #[default]
    Public,
    My,
    User,
}
