use std::borrow::Borrow;
use std::rc::Rc;

use futures::{self, Future, Stream};
use hyper::{self, Body, Request};
use serde_json;

use super::{configuration, Error};

pub struct AuthenticationApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> AuthenticationApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AuthenticationApiClient<C> {
        AuthenticationApiClient { configuration }
    }
}

pub trait AuthenticationApi {
    fn login_post(
        &self,
        authentication_string: crate::models::Auth,
    ) -> Box<dyn Future<Item = crate::models::Token, Error = Error<serde_json::Value>>>;
    fn refresh_token_get(
        &self,
    ) -> Box<dyn Future<Item = crate::models::Token, Error = Error<serde_json::Value>>>;
}

impl<C: 'static + hyper::client::connect::Connect> AuthenticationApi
    for AuthenticationApiClient<C>
{
    fn login_post(
        &self,
        authentication_string: crate::models::Auth,
    ) -> Box<dyn Future<Item = crate::models::Token, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let req = Request::post(format!("{}/login", configuration.base_path))
            .header(
                hyper::header::USER_AGENT,
                configuration.user_agent.as_ref().unwrap(),
            )
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .header(hyper::header::ACCEPT, "application/json")
            .body(Body::from(
                serde_json::to_string(&authentication_string).unwrap(),
            ))
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
                    let parsed: Result<crate::models::Token, _> = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn refresh_token_get(
        &self,
    ) -> Box<dyn Future<Item = crate::models::Token, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let authorization = match configuration.token.clone() {
            Some(v) => v,
            None => {
                panic!("You need to provide an authorization token before making this API call")
            }
        };

        let req = Request::get(format!("{}/refresh_token", configuration.base_path))
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
                    let parsed: Result<crate::models::Token, _> = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }
}
