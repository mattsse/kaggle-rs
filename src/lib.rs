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

mod archive;
pub mod client;
mod error;
pub mod models;
mod none_as_empty;
pub mod query;
pub mod request;

pub use client::{Authentication, KaggleApiClient, KaggleApiClientBuilder};
