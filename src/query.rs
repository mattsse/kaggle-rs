use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum PushKernelTypes {
    Script,
    Notebook,
}

#[derive(Debug, Clone, Serialize)]
pub enum PushLanguageTypes {
    Python,
    R,
    Rmarkdown,
}

#[derive(Debug, Clone, Serialize)]
pub enum ListLanguages {
    All,
    Python,
    R,
    Sqlite,
    Julia,
}

impl Default for ListLanguages {
    fn default() -> Self {
        ListLanguages::All
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum ListKernelTypes {
    All,
    Script,
    Notebook,
}

impl Default for ListKernelTypes {
    fn default() -> Self {
        ListKernelTypes::All
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum ListOutputTypes {
    All,
    Visualization,
    Data,
}

impl Default for ListOutputTypes {
    fn default() -> Self {
        ListOutputTypes::All
    }
}

#[derive(Debug, Clone, Serialize)]
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

/// Competitoins valid types
#[derive(Debug, Clone, Serialize)]
pub enum CompetitionGroups {
    General,
    Entered,
    InClass,
}

#[derive(Debug, Clone, Serialize)]
pub enum CompetitionCategories {
    All,
    Featured,
    Research,
    Recruitment,
    GettingStarted,
    Masters,
    Playground,
}

impl Default for CompetitionCategories {
    fn default() -> Self {
        CompetitionCategories::All
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum CompetitionSortBy {
    Grouped,
    Prize,
    EarliestDeadline,
    LatestDeadline,
    NumberOfTeams,
    RecentlyCreated,
}

/// Datasets valid types
#[derive(Debug, Clone, Serialize)]
pub enum DatasetFileTypes {
    All,
    Csv,
    Sqlite,
    Json,
    BigQuery,
}

impl Default for DatasetFileTypes {
    fn default() -> Self {
        DatasetFileTypes::All
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum DatasetLicenseNames {
    All,
    Cc,
    Gpl,
    Odb,
    Other,
}

impl Default for DatasetLicenseNames {
    fn default() -> Self {
        DatasetLicenseNames::All
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum DatasetSortBys {
    Hottest,
    Votes,
    Updated,
    Active,
    Published,
}
