use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

use futures;
use futures::{Future, Stream};
use hyper;
use serde_json;

use hyper::header::USER_AGENT;

use super::{configuration, Error};

pub struct UpdatesApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> UpdatesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> UpdatesApiClient<C> {
        UpdatesApiClient { configuration }
    }
}

pub trait UpdatesApi {
    fn updated_query_get(
        &self,
        from_time: &str,
        to_time: &str,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::UpdateData, Error = Error<serde_json::Value>>>;
    fn updated_query_params_get(
        &self,
    ) -> Box<
        dyn Future<Item = crate::models::UpdateDataQueryParams, Error = Error<serde_json::Value>>,
    >;
}

impl<C: hyper::client::connect::Connect> UpdatesApi for UpdatesApiClient<C> {
    fn updated_query_get(
        &self,
        from_time: &str,
        to_time: &str,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::UpdateData, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_headers.insert("Authorization".to_owned(), val);
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("fromTime", &from_time.to_string());
            query.append_pair("toTime", &to_time.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/updated/query?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(USER_AGENT);
        }

        {
            let headers = req.headers_mut();
            headers.set_raw("Accept-Language", accept_language);
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(Error::from)
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(Error::from)
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<crate::models::UpdateData, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn updated_query_params_get(
        &self,
    ) -> Box<
        dyn Future<Item = crate::models::UpdateDataQueryParams, Error = Error<serde_json::Value>>,
    > {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_headers.insert("Authorization".to_owned(), val);
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
            "{}/updated/query/params?{}",
            configuration.base_path, query_string
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(USER_AGENT);
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(Error::from)
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(Error::from)
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<crate::models::UpdateDataQueryParams, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }
}
