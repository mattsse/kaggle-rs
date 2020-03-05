use reqwest::Url;

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl ApiKey {}

#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: Url,
    pub user_agent: String,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    // TODO: take an oauth2 token source, similar to the go one
}

impl Config {
    /// Convenience method to create a [`ConfigurationBuilder`]
    #[inline]
    pub fn builder() -> ConfigBuilder {
        unimplemented!()
    }

    pub fn new() -> Self {
        Config {
            base_url: "https://www.kaggle.com/api/v1".parse().unwrap(),
            user_agent: "kaggele-rs/1/rust".to_string(),
            basic_auth: None,
            oauth_access_token: None,
            api_key: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConfigBuilder {}

impl ConfigBuilder {
    pub fn build(self) -> Config {
        unimplemented!()
    }
}
