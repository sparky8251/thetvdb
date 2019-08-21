use std::borrow::Borrow;
use std::rc::Rc;

use futures::{self, Future, Stream};
use hyper::{self, Body, Request};
use serde_json;

use super::{configuration, Error};

pub struct SearchApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> SearchApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> SearchApiClient<C> {
        SearchApiClient { configuration }
    }
}

pub trait SearchApi {
    fn search_series_get(
        &self,
        search: crate::models::SeriesSearchQueryParams,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::SeriesSearchResults, Error = Error<serde_json::Value>>>;
}

impl<C: 'static + hyper::client::connect::Connect> SearchApi for SearchApiClient<C> {
    fn search_series_get(
        &self,
        search: crate::models::SeriesSearchQueryParams,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::SeriesSearchResults, Error = Error<serde_json::Value>>>
    {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let authorization = match &configuration.token {
            Some(v) => {
                let p = v.prefix.clone();
                let t = v.token.clone();
                format!("{} {}", p, t)
            }
            None => {
                panic!("You need to provide an authorization token before making this API call")
            }
        };

        let req = Request::get(format!(
            "{}/search/series{}",
            configuration.base_path, search
        ))
        .header(
            hyper::header::USER_AGENT,
            configuration.user_agent.as_ref().unwrap(),
        )
        .header(hyper::header::ACCEPT, "application/json")
        .header(hyper::header::AUTHORIZATION, authorization)
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
                    let parsed: Result<crate::models::SeriesSearchResults, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }
}
