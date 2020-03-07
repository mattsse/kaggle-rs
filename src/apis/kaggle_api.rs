use futures;
use futures::{Future, Stream};
use hyper;
use hyper::header::UserAgent;
use serde_json;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

use super::{configuration, Error};

pub struct KaggleApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> KaggleApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> KaggleApiClient<C> {
        KaggleApiClient {
            configuration: configuration,
        }
    }
}

pub trait KaggleApi {
    fn competition_download_leaderboard(
        &self,
        id: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn competition_view_leaderboard(
        &self,
        id: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn competitions_data_download_file(
        &self,
        id: &str,
        file_name: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn competitions_data_download_files(
        &self,
        id: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn competitions_data_list_files(
        &self,
        id: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn competitions_list(
        &self,
        group: &str,
        category: &str,
        sort_by: &str,
        page: i32,
        search: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn competitions_submissions_list(
        &self,
        id: &str,
        page: i32,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn competitions_submissions_submit(
        &self,
        blob_file_tokens: &str,
        submission_description: &str,
        id: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn competitions_submissions_upload(
        &self,
        file: ::models::File,
        guid: &str,
        content_length: i32,
        last_modified_date_utc: i32,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn competitions_submissions_url(
        &self,
        id: &str,
        content_length: i32,
        last_modified_date_utc: i32,
        file_name: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn datasets_create_new(
        &self,
        dataset_new_request: ::models::DatasetNewRequest,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn datasets_create_version(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        dataset_new_version_request: ::models::DatasetNewVersionRequest,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn datasets_create_version_by_id(
        &self,
        id: i32,
        dataset_new_version_request: ::models::DatasetNewVersionRequest,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn datasets_download(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        dataset_version_number: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn datasets_download_file(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        file_name: &str,
        dataset_version_number: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn datasets_list(
        &self,
        group: &str,
        sort_by: &str,
        size: &str,
        filetype: &str,
        license: &str,
        tagids: &str,
        search: &str,
        user: &str,
        page: i32,
        max_size: i64,
        min_size: i64,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn datasets_list_files(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn datasets_status(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn datasets_upload_file(
        &self,
        file_name: &str,
        content_length: i32,
        last_modified_date_utc: i32,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn datasets_view(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn kernel_output(
        &self,
        user_name: &str,
        kernel_slug: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn kernel_pull(
        &self,
        user_name: &str,
        kernel_slug: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn kernel_push(
        &self,
        kernel_push_request: ::models::KernelPushRequest,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn kernel_status(
        &self,
        user_name: &str,
        kernel_slug: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn kernels_list(
        &self,
        page: i32,
        page_size: i32,
        search: &str,
        group: &str,
        user: &str,
        language: &str,
        kernel_type: &str,
        output_type: &str,
        sort_by: &str,
        dataset: &str,
        competition: &str,
        parent_kernel: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn metadata_get(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
    fn metadata_post(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        settings: ::models::DatasetUpdateSettingsRequest,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> KaggleApi for KaggleApiClient<C> {
    fn competition_download_leaderboard(
        &self,
        id: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/competitions/{id}/leaderboard/download?{}",
            configuration.base_path,
            query_string,
            id = id
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn competition_view_leaderboard(
        &self,
        id: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/competitions/{id}/leaderboard/view?{}",
            configuration.base_path,
            query_string,
            id = id
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn competitions_data_download_file(
        &self,
        id: &str,
        file_name: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/competitions/data/download/{id}/{fileName}?{}",
            configuration.base_path,
            query_string,
            id = id,
            fileName = file_name
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn competitions_data_download_files(
        &self,
        id: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/competitions/data/download-all/{id}?{}",
            configuration.base_path,
            query_string,
            id = id
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn competitions_data_list_files(
        &self,
        id: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/competitions/data/list/{id}?{}",
            configuration.base_path,
            query_string,
            id = id
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn competitions_list(
        &self,
        group: &str,
        category: &str,
        sort_by: &str,
        page: i32,
        search: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("group", &group.to_string());
            query.append_pair("category", &category.to_string());
            query.append_pair("sortBy", &sort_by.to_string());
            query.append_pair("page", &page.to_string());
            query.append_pair("search", &search.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/competitions/list?{}",
            configuration.base_path, query_string
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn competitions_submissions_list(
        &self,
        id: &str,
        page: i32,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("page", &page.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/competitions/submissions/list/{id}?{}",
            configuration.base_path,
            query_string,
            id = id
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn competitions_submissions_submit(
        &self,
        blob_file_tokens: &str,
        submission_description: &str,
        id: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/competitions/submissions/submit/{id}?{}",
            configuration.base_path,
            query_string,
            id = id
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn competitions_submissions_upload(
        &self,
        file: ::models::File,
        guid: &str,
        content_length: i32,
        last_modified_date_utc: i32,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/competitions/submissions/upload/{guid}/{contentLength}/{lastModifiedDateUtc}?{}",
            configuration.base_path,
            query_string,
            guid = guid,
            contentLength = content_length,
            lastModifiedDateUtc = last_modified_date_utc
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn competitions_submissions_url(
        &self,
        id: &str,
        content_length: i32,
        last_modified_date_utc: i32,
        file_name: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/competitions/{id}/submissions/url/{contentLength}/{lastModifiedDateUtc}?{}",
            configuration.base_path,
            query_string,
            id = id,
            contentLength = content_length,
            lastModifiedDateUtc = last_modified_date_utc
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn datasets_create_new(
        &self,
        dataset_new_request: ::models::DatasetNewRequest,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/datasets/create/new?{}",
            configuration.base_path, query_string
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&dataset_new_request).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut()
            .set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn datasets_create_version(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        dataset_new_version_request: ::models::DatasetNewVersionRequest,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/datasets/create/version/{ownerSlug}/{datasetSlug}?{}",
            configuration.base_path,
            query_string,
            ownerSlug = owner_slug,
            datasetSlug = dataset_slug
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&dataset_new_version_request).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut()
            .set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn datasets_create_version_by_id(
        &self,
        id: i32,
        dataset_new_version_request: ::models::DatasetNewVersionRequest,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/datasets/create/version/{id}?{}",
            configuration.base_path,
            query_string,
            id = id
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&dataset_new_version_request).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut()
            .set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn datasets_download(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        dataset_version_number: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("datasetVersionNumber", &dataset_version_number.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/datasets/download/{ownerSlug}/{datasetSlug}?{}",
            configuration.base_path,
            query_string,
            ownerSlug = owner_slug,
            datasetSlug = dataset_slug
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn datasets_download_file(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        file_name: &str,
        dataset_version_number: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("datasetVersionNumber", &dataset_version_number.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/datasets/download/{ownerSlug}/{datasetSlug}/{fileName}?{}",
            configuration.base_path,
            query_string,
            ownerSlug = owner_slug,
            datasetSlug = dataset_slug,
            fileName = file_name
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn datasets_list(
        &self,
        group: &str,
        sort_by: &str,
        size: &str,
        filetype: &str,
        license: &str,
        tagids: &str,
        search: &str,
        user: &str,
        page: i32,
        max_size: i64,
        min_size: i64,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("group", &group.to_string());
            query.append_pair("sortBy", &sort_by.to_string());
            query.append_pair("size", &size.to_string());
            query.append_pair("filetype", &filetype.to_string());
            query.append_pair("license", &license.to_string());
            query.append_pair("tagids", &tagids.to_string());
            query.append_pair("search", &search.to_string());
            query.append_pair("user", &user.to_string());
            query.append_pair("page", &page.to_string());
            query.append_pair("maxSize", &max_size.to_string());
            query.append_pair("minSize", &min_size.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/datasets/list?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn datasets_list_files(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/datasets/list/{ownerSlug}/{datasetSlug}?{}",
            configuration.base_path,
            query_string,
            ownerSlug = owner_slug,
            datasetSlug = dataset_slug
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn datasets_status(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/datasets/status/{ownerSlug}/{datasetSlug}?{}",
            configuration.base_path,
            query_string,
            ownerSlug = owner_slug,
            datasetSlug = dataset_slug
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn datasets_upload_file(
        &self,
        file_name: &str,
        content_length: i32,
        last_modified_date_utc: i32,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/datasets/upload/file/{contentLength}/{lastModifiedDateUtc}?{}",
            configuration.base_path,
            query_string,
            contentLength = content_length,
            lastModifiedDateUtc = last_modified_date_utc
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn datasets_view(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/datasets/view/{ownerSlug}/{datasetSlug}?{}",
            configuration.base_path,
            query_string,
            ownerSlug = owner_slug,
            datasetSlug = dataset_slug
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn kernel_output(
        &self,
        user_name: &str,
        kernel_slug: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("userName", &user_name.to_string());
            query.append_pair("kernelSlug", &kernel_slug.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/kernels/output?{}",
            configuration.base_path, query_string
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn kernel_pull(
        &self,
        user_name: &str,
        kernel_slug: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("userName", &user_name.to_string());
            query.append_pair("kernelSlug", &kernel_slug.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/kernels/pull?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn kernel_push(
        &self,
        kernel_push_request: ::models::KernelPushRequest,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/kernels/push?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&kernel_push_request).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut()
            .set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn kernel_status(
        &self,
        user_name: &str,
        kernel_slug: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("userName", &user_name.to_string());
            query.append_pair("kernelSlug", &kernel_slug.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/kernels/status?{}",
            configuration.base_path, query_string
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn kernels_list(
        &self,
        page: i32,
        page_size: i32,
        search: &str,
        group: &str,
        user: &str,
        language: &str,
        kernel_type: &str,
        output_type: &str,
        sort_by: &str,
        dataset: &str,
        competition: &str,
        parent_kernel: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("page", &page.to_string());
            query.append_pair("pageSize", &page_size.to_string());
            query.append_pair("search", &search.to_string());
            query.append_pair("group", &group.to_string());
            query.append_pair("user", &user.to_string());
            query.append_pair("language", &language.to_string());
            query.append_pair("kernelType", &kernel_type.to_string());
            query.append_pair("outputType", &output_type.to_string());
            query.append_pair("sortBy", &sort_by.to_string());
            query.append_pair("dataset", &dataset.to_string());
            query.append_pair("competition", &competition.to_string());
            query.append_pair("parentKernel", &parent_kernel.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/kernels/list?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn metadata_get(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/datasets/metadata/{ownerSlug}/{datasetSlug}?{}",
            configuration.base_path,
            query_string,
            ownerSlug = owner_slug,
            datasetSlug = dataset_slug
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }

    fn metadata_post(
        &self,
        owner_slug: &str,
        dataset_slug: &str,
        settings: ::models::DatasetUpdateSettingsRequest,
    ) -> Box<Future<Item = ::models::Result, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(hyper::header::Basic {
                username: auth_conf.0.to_string(),
                password: auth_conf.1.to_string(),
            });
            auth_headers.insert("Authorization".to_string(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/datasets/metadata/{ownerSlug}/{datasetSlug}?{}",
            configuration.base_path,
            query_string,
            ownerSlug = owner_slug,
            datasetSlug = dataset_slug
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&settings).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut()
            .set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(|e| Error::from(e))
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(|e| Error::from(e))
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<::models::Result, _> = serde_json::from_slice(&body);
                    parsed.map_err(|e| Error::from(e))
                }),
        )
    }
}
