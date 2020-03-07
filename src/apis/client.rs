use hyper;
use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
    configuration: Rc<Configuration<C>>,
    kaggle_api: Box<::apis::KaggleApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
    pub fn new(configuration: Configuration<C>) -> APIClient<C> {
        let rc = Rc::new(configuration);

        APIClient {
            configuration: rc.clone(),
            kaggle_api: Box::new(::apis::KaggleApiClient::new(rc.clone())),
        }
    }

    pub fn kaggle_api(&self) -> &::apis::KaggleApi {
        self.kaggle_api.as_ref()
    }
}
