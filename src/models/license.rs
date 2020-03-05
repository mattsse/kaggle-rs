use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    /// Name of the license
    name: String,
}

impl License {
    pub fn new<T: ToString>(name: T) -> License {
        License {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

pub enum Licenses {
    /// CC0-1.0
    Cc010,
    /// CC-BY-SA-4.0
    CcBySa40,
    /// GPL-2.0
    Gpl20,
    /// ODbL-1.0
    OdbL10,
    /// CC-BY-NC-SA-4.0
    CcByNcSa40,
    /// unknown
    Unknown,
    /// DbCL-1.0
    DbCl10,
    /// CC-BY-SA-3.0
    CcBySa30,
    /// copyright-authors
    CopyrightAuthors,
    /// other
    Other,
    /// reddit-api
    RedditApi,
    /// world-bank
    WorldBank,
}
