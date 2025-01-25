//! Unoffical implementations of the Kaggle api: https://github.com/Kaggle/kaggle-api
//!
//!
//! # Authentication (see also https://github.com/Kaggle/kaggle-api#api-credentials)
//!
//! To use the Kaggle API, a Kaggle account and API Token is required: https://www.kaggle.com/.
//! To generate a API Token for your account, visit `https://www.kaggle.com/<username>/account` and `Create API Token`. By default this crate assumes that you put the `kaggle.json` at `~/.kaggle/kaggle.json` (on Windows in at `C:\Users\<Windows-username>\.kaggle\kaggle.json`):
//!
//! ```
//!  kaggle::Authentication::default();
//! ```
//!
//! However you can also point to an other location
//!
//! ```
//! kaggle::Authentication::with_config_file("<path>/kaggle.json");
//! ```
//!
//! Other authentication mechanisms are:
//!
//! Direct:
//! ```
//! kaggle::Authentication::with_credentials("user_name", "key");
//! ```
//! From environment variables
//!
//! ```sh
//! export KAGGLE_USERNAME=datadinosaur
//! export KAGGLE_KEY=xxxxxxxxxxxxxx
//! ```
//! ```
//! kaggle::Authentication::Env;
//! ```

pub mod archive;
pub mod client;
mod error;
pub mod models;
mod none_as_empty;
pub mod query;
pub mod request;

pub use client::{Authentication, KaggleApiClient, KaggleApiClientBuilder};

#[cfg(test)]
mod tests {
    use crate::KaggleApiClient;
    use std::path::Path;

    #[tokio::test]
    #[ignore]
    async fn read_dataset_metadata() -> anyhow::Result<()> {
        env_logger::init();
        let kaggle = KaggleApiClient::builder().build()?;
        let _metadata = kaggle
            .metadata_get("mczielinski/bitcoin-historical-data")
            .await?;
        Ok(())
    }
    #[tokio::test]
    #[ignore]
    async fn download_dataset() -> anyhow::Result<()> {
        env_logger::init();
        let kaggle = KaggleApiClient::builder().build()?;
        let _metadata = kaggle
            .dataset_download_all_files(
                "mczielinski/bitcoin-historical-data",
                Some(Path::new("/tmp/").to_path_buf()),
                None,
            )
            .await?;
        Ok(())
    }
}
