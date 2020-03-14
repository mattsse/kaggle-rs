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

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Language {
    All,
    Python,
    R,
    Sqlite,
    Julia,
}

impl Default for Language {
    fn default() -> Self {
        Language::All
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum KernelType {
    All,
    Script,
    Notebook,
}

impl Default for KernelType {
    fn default() -> Self {
        KernelType::All
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OutputType {
    All,
    Visualization,
    Data,
}

impl Default for OutputType {
    fn default() -> Self {
        OutputType::All
    }
}

/// How to sort the result
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SortBy {
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

impl Default for SortBy {
    fn default() -> Self {
        SortBy::Hotness
    }
}

/// Competitoins valid types
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CompetitionGroup {
    General,
    Entered,
    InClass,
}

impl Default for CompetitionGroup {
    fn default() -> Self {
        CompetitionGroup::General
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CompetitionCategory {
    All,
    Featured,
    Research,
    Recruitment,
    GettingStarted,
    Masters,
    Playground,
}

impl Default for CompetitionCategory {
    fn default() -> Self {
        CompetitionCategory::All
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CompetitionSortBy {
    Grouped,
    Prize,
    EarliestDeadline,
    LatestDeadline,
    NumberOfTeams,
    RecentlyCreated,
}

impl Default for CompetitionSortBy {
    fn default() -> Self {
        CompetitionSortBy::LatestDeadline
    }
}

/// Datasets valid types
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DatasetFileType {
    All,
    Csv,
    Sqlite,
    Json,
    BigQuery,
}

impl Default for DatasetFileType {
    fn default() -> Self {
        DatasetFileType::All
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DatasetLicenseName {
    All,
    Cc,
    Gpl,
    Odb,
    Other,
}

impl Default for DatasetLicenseName {
    fn default() -> Self {
        DatasetLicenseName::All
    }
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
pub enum Group {
    Everyone,
    Profile,
}

impl Default for Group {
    fn default() -> Self {
        Group::Everyone
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum DatasetGroup {
    Public,
    My,
    User,
}

impl Default for DatasetGroup {
    fn default() -> Self {
        DatasetGroup::Public
    }
}
