use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum License {
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
// f.write_str(
impl fmt::Display for License {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            License::Cc010 => f.write_str("CC0-1.0"),
            License::CcBySa40 => f.write_str("CC-BY-SA-4.0"),
            License::Gpl20 => f.write_str("GPL-2.0"),
            License::OdbL10 => f.write_str("ODbL-1.0"),
            License::CcByNcSa40 => f.write_str("CC-BY-NC-SA-4.0"),
            License::Unknown => f.write_str("unknown"),
            License::DbCl10 => f.write_str("DbCL-1.0"),
            License::CcBySa30 => f.write_str("CC-BY-SA-3.0"),
            License::CopyrightAuthors => f.write_str("copyright-authors"),
            License::Other => f.write_str("other"),
            License::RedditApi => f.write_str("reddit-api"),
            License::WorldBank => f.write_str("world-bank"),
        }
    }
}

impl FromStr for License {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let license = match s {
            "CC0-1.0" => License::Cc010,
            "CC-BY-SA-4.0" => License::CcBySa40,
            "GPL-2.0" => License::Gpl20,
            "ODbL-1.0" => License::OdbL10,
            "CC-BY-NC-SA-4.0" => License::CcByNcSa40,
            "unknown" => License::Unknown,
            "DbCL-1.0" => License::DbCl10,
            "CC-BY-SA-3.0" => License::CcBySa30,
            "copyright-authors" => License::CopyrightAuthors,
            "other" => License::Other,
            "reddit-api" => License::RedditApi,
            "world-bank" => License::WorldBank,
            _ => License::Other,
        };
        Ok(license)
    }
}

impl<'de> Deserialize<'de> for License {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Field {
            name: String,
        }
        let f = Field::deserialize(deserializer)?;
        License::from_str(&f.name).map_err(serde::de::Error::custom)
    }
}

impl Serialize for License {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("License", 13)?;
        state.serialize_field("name", &self.to_string())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ser_license() {
        assert_eq!(
            r#"{"name":"other"}"#,
            serde_json::to_string(&License::Other).unwrap()
        );
    }

    #[test]
    fn de_license() {
        assert_eq!(
            serde_json::from_str::<License>(r#"{"name":"other"}"#).unwrap(),
            License::Other
        );
    }
}
