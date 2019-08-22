use std::borrow::Borrow;
use std::rc::Rc;

use futures::{self, Future, Stream};
use hyper::{self, Body, Request};
use serde_json;

use super::{configuration, Error};

pub struct LanguagesApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> LanguagesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> LanguagesApiClient<C> {
        LanguagesApiClient { configuration }
    }
}

pub trait LanguagesApi {
    fn languages_get(
        &self,
    ) -> Box<dyn Future<Item = crate::models::LanguageData, Error = Error<serde_json::Value>>>;
    fn languages_id_get(
        &self,
        id: u64,
    ) -> Box<dyn Future<Item = crate::models::LanguageDataSingle, Error = Error<serde_json::Value>>>;
}

impl<C: 'static + hyper::client::connect::Connect> LanguagesApi for LanguagesApiClient<C> {
    fn languages_get(
        &self,
    ) -> Box<dyn Future<Item = crate::models::LanguageData, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let authorization = match configuration.token.clone() {
            Some(v) => v,
            None => {
                panic!("You need to provide an authorization token before making this API call")
            }
        };

        let req = Request::get(format!("{}/languages", configuration.base_path))
            .header(
                hyper::header::USER_AGENT,
                configuration.user_agent.as_ref().unwrap(),
            )
            .header(hyper::header::ACCEPT, "application/json")
            .header(hyper::header::AUTHORIZATION, authorization.to_string())
            .body(Body::empty())
            .expect("request builder");

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(Error::from)
                .and_then(|resp| {
                    let status = resp.status();
                    resp.into_body()
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
                    let parsed: Result<crate::models::LanguageData, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn languages_id_get(
        &self,
        id: u64,
    ) -> Box<dyn Future<Item = crate::models::LanguageDataSingle, Error = Error<serde_json::Value>>>
    {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let authorization = match configuration.token.clone() {
            Some(v) => v,
            None => {
                panic!("You need to provide an authorization token before making this API call")
            }
        };

        let req = Request::get(format!("{}/languages/{}", configuration.base_path, id))
            .header(
                hyper::header::USER_AGENT,
                configuration.user_agent.as_ref().unwrap(),
            )
            .header(hyper::header::ACCEPT, "application/json")
            .header(hyper::header::AUTHORIZATION, authorization.to_string())
            .body(Body::empty())
            .expect("request builder");

        // send request
        Box::new(
            configuration
                .client
                .request(dbg!(req))
                .map_err(Error::from)
                .and_then(|resp| {
                    let status = resp.status();
                    resp.into_body()
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
                    let parsed: Result<crate::models::LanguageDataSingle, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }
}
