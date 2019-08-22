use std::borrow::Borrow;
use std::rc::Rc;

use futures::{self, Future, Stream};
use hyper::{self, Body, Request};
use serde_json;

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
        times: crate::models::UpdateDataQueryParams,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::UpdateData, Error = Error<serde_json::Value>>>;
}

impl<C: 'static + hyper::client::connect::Connect> UpdatesApi for UpdatesApiClient<C> {
    fn updated_query_get(
        &self,
        times: crate::models::UpdateDataQueryParams,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::UpdateData, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let authorization = match configuration.token.clone() {
            Some(v) => v,
            None => {
                panic!("You need to provide an authorization token before making this API call")
            }
        };

        let req = Request::get(format!(
            "{}/updated/query{}",
            configuration.base_path, times
        ))
        .header(
            hyper::header::USER_AGENT,
            configuration.user_agent.as_ref().unwrap(),
        )
        .header(hyper::header::ACCEPT, "application/json")
        .header(hyper::header::AUTHORIZATION, authorization.to_string())
        .header("Accept-Language", accept_language)
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
                    let parsed: Result<crate::models::UpdateData, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }
}
