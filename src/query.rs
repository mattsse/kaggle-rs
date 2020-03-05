use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PushKernelType {
    Script,
    Notebook,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PushLanguageType {
    Python,
    R,
    Rmarkdown,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ListLanguage {
    All,
    Python,
    R,
    Sqlite,
    Julia,
}

impl Default for ListLanguage {
    fn default() -> Self {
        ListLanguage::All
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ListKernelType {
    All,
    Script,
    Notebook,
}

impl Default for ListKernelType {
    fn default() -> Self {
        ListKernelType::All
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ListOutputType {
    All,
    Visualization,
    Data,
}

impl Default for ListOutputType {
    fn default() -> Self {
        ListOutputType::All
    }
}

/// How to sort the result
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ListSortBy {
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

impl Default for ListSortBy {
    fn default() -> Self {
        ListSortBy::Hotness
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
