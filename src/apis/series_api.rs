use std::borrow::Borrow;
use std::rc::Rc;

use futures::{self, Future, Stream};
use hyper::{self, Body, Request};
use serde_json;

use super::{configuration, Error};

pub struct SeriesApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> SeriesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> SeriesApiClient<C> {
        SeriesApiClient { configuration }
    }
}

pub trait SeriesApi {
    fn series_id_actors_get(
        &self,
        id: i64,
    ) -> Box<dyn Future<Item = crate::models::SeriesActors, Error = Error<serde_json::Value>>>;
    fn series_id_episodes_get(
        &self,
        id: i64,
        page: Option<i64>,
    ) -> Box<dyn Future<Item = crate::models::SeriesEpisodes, Error = Error<serde_json::Value>>>;
    #[allow(clippy::too_many_arguments)]
    fn series_id_episodes_query_get(
        &self,
        id: i64,
        absolute_number: &str,
        aired_season: &str,
        aired_episode: &str,
        dvd_season: &str,
        dvd_episode: &str,
        imdb_id: &str,
        page: Option<i64>,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::SeriesEpisodesQuery, Error = Error<serde_json::Value>>>;
    // fn series_id_episodes_summary_get(
    //     &self,
    //     id: i64,
    // ) -> Box<
    //     dyn Future<Item = crate::models::SeriesEpisodesSummary, Error = Error<serde_json::Value>>,
    // >;
    // fn series_id_filter_get(
    //     &self,
    //     id: i64,
    //     keys: &str,
    //     accept_language: &str,
    // ) -> Box<dyn Future<Item = crate::models::SeriesData, Error = Error<serde_json::Value>>>;
    fn series_id_get(
        &self,
        id: i64,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::SeriesData, Error = Error<serde_json::Value>>>;
    // fn series_id_head(
    //     &self,
    //     id: i64,
    //     accept_language: &str,
    // ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    // fn series_id_images_get(
    //     &self,
    //     id: i64,
    //     accept_language: &str,
    // ) -> Box<dyn Future<Item = crate::models::SeriesImagesCounts, Error = Error<serde_json::Value>>>;
    // fn series_id_images_query_get(
    //     &self,
    //     id: i64,
    //     key_type: &str,
    //     resolution: &str,
    //     sub_key: &str,
    //     accept_language: &str,
    // ) -> Box<
    //     dyn Future<Item = crate::models::SeriesImageQueryResults, Error = Error<serde_json::Value>>,
    // >;
}

impl<C: 'static + hyper::client::connect::Connect> SeriesApi for SeriesApiClient<C> {
    fn series_id_actors_get(
        &self,
        id: i64,
    ) -> Box<dyn Future<Item = crate::models::SeriesActors, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let authorization = match configuration.token.clone() {
            Some(v) => v,
            None => {
                panic!("You need to provide an authorization token before making this API call")
            }
        };

        let req = Request::get(format!("{}/series/{}/actors", configuration.base_path, id))
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
                    let parsed: Result<crate::models::SeriesActors, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn series_id_episodes_get(
        &self,
        id: i64,
        page: Option<i64>,
    ) -> Box<dyn Future<Item = crate::models::SeriesEpisodes, Error = Error<serde_json::Value>>>
    {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let authorization = match configuration.token.clone() {
            Some(v) => v,
            None => {
                panic!("You need to provide an authorization token before making this API call")
            }
        };

        let page = match page {
            Some(v) => format!("?page={}", v),
            None => format!(""),
        };

        let req = Request::get(format!(
            "{}/series/{}/episodes{}",
            configuration.base_path, id, page
        ))
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
                    let parsed: Result<crate::models::SeriesEpisodes, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn series_id_episodes_query_get(
        &self,
        id: i64,
        _absolute_number: &str,
        _aired_season: &str,
        _aired_episode: &str,
        _dvd_season: &str,
        _dvd_episode: &str,
        _imdb_id: &str,
        _page: Option<i64>,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::SeriesEpisodesQuery, Error = Error<serde_json::Value>>>
    {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let authorization = match configuration.token.clone() {
            Some(v) => v,
            None => {
                panic!("You need to provide an authorization token before making this API call")
            }
        };

        let req = Request::get(format!(
            "{}/series/{}/episodes/query",
            configuration.base_path, id
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
                    let parsed: Result<crate::models::SeriesEpisodesQuery, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    // fn series_id_episodes_summary_get(
    //     &self,
    //     id: i64,
    // ) -> Box<
    //     dyn Future<Item = crate::models::SeriesEpisodesSummary, Error = Error<serde_json::Value>>,
    // > {
    //     let configuration: &configuration::Configuration<C> = self.configuration.borrow();
    //
    //     let mut auth_headers = HashMap::<String, String>::new();
    //     let auth_query = HashMap::<String, String>::new();
    //     if let Some(ref apikey) = configuration.api_key {
    //         let key = apikey.key.clone();
    //         let val = match apikey.prefix {
    //             Some(ref prefix) => format!("{} {}", prefix, key),
    //             None => key,
    //         };
    //         auth_headers.insert("Authorization".to_owned(), val);
    //     };
    //     let method = hyper::Method::Get;
    //
    //     let query_string = {
    //         let mut query = ::url::form_urlencoded::Serializer::new(String::new());
    //         for (key, val) in &auth_query {
    //             query.append_pair(key, val);
    //         }
    //         query.finish()
    //     };
    //     let uri_str = format!(
    //         "{}/series/{id}/episodes/summary?{}",
    //         configuration.base_path,
    //         query_string,
    //         id = id
    //     );
    //
    //     // TODO(farcaller): handle error
    //     // if let Err(e) = uri {
    //     //     return Box::new(futures::future::err(e));
    //     // }
    //     let uri: hyper::Uri = uri_str.parse().unwrap();
    //
    //     let mut req = hyper::Request::new(method, uri);
    //
    //     if let Some(ref user_agent) = configuration.user_agent {
    //         req.headers_mut().set(USER_AGENT);
    //     }
    //
    //     for (key, val) in auth_headers {
    //         req.headers_mut().set_raw(key, val);
    //     }
    //
    //     // send request
    //     Box::new(
    //         configuration
    //             .client
    //             .request(req)
    //             .map_err(Error::from)
    //             .and_then(|resp| {
    //                 let status = resp.status();
    //                 resp.body()
    //                     .concat2()
    //                     .and_then(move |body| Ok((status, body)))
    //                     .map_err(Error::from)
    //             })
    //             .and_then(|(status, body)| {
    //                 if status.is_success() {
    //                     Ok(body)
    //                 } else {
    //                     Err(Error::from((status, &*body)))
    //                 }
    //             })
    //             .and_then(|body| {
    //                 let parsed: Result<crate::models::SeriesEpisodesSummary, _> =
    //                     serde_json::from_slice(&body);
    //                 parsed.map_err(Error::from)
    //             }),
    //     )
    // }

    fn series_id_get(
        &self,
        id: i64,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::SeriesData, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let authorization = match configuration.token.clone() {
            Some(v) => v,
            None => {
                panic!("You need to provide an authorization token before making this API call")
            }
        };

        let req = Request::get(format!("{}/series/{}", configuration.base_path, id))
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
                    let parsed: Result<crate::models::SeriesData, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    // fn series_id_head(
    //     &self,
    //     id: i64,
    //     accept_language: &str,
    // ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
    //     let configuration: &configuration::Configuration<C> = self.configuration.borrow();
    //
    //     let mut auth_headers = HashMap::<String, String>::new();
    //     let auth_query = HashMap::<String, String>::new();
    //     if let Some(ref apikey) = configuration.api_key {
    //         let key = apikey.key.clone();
    //         let val = match apikey.prefix {
    //             Some(ref prefix) => format!("{} {}", prefix, key),
    //             None => key,
    //         };
    //         auth_headers.insert("Authorization".to_owned(), val);
    //     };
    //     let method = hyper::Method::Head;
    //
    //     let query_string = {
    //         let mut query = ::url::form_urlencoded::Serializer::new(String::new());
    //         for (key, val) in &auth_query {
    //             query.append_pair(key, val);
    //         }
    //         query.finish()
    //     };
    //     let uri_str = format!(
    //         "{}/series/{id}?{}",
    //         configuration.base_path,
    //         query_string,
    //         id = id
    //     );
    //
    //     // TODO(farcaller): handle error
    //     // if let Err(e) = uri {
    //     //     return Box::new(futures::future::err(e));
    //     // }
    //     let uri: hyper::Uri = uri_str.parse().unwrap();
    //
    //     let mut req = hyper::Request::new(method, uri);
    //
    //     if let Some(ref user_agent) = configuration.user_agent {
    //         req.headers_mut().set(USER_AGENT);
    //     }
    //
    //     {
    //         let headers = req.headers_mut();
    //         headers.set_raw("Accept-Language", accept_language);
    //     }
    //
    //     for (key, val) in auth_headers {
    //         req.headers_mut().set_raw(key, val);
    //     }
    //
    //     // send request
    //     Box::new(
    //         configuration
    //             .client
    //             .request(req)
    //             .map_err(Error::from)
    //             .and_then(|resp| {
    //                 let status = resp.status();
    //                 resp.body()
    //                     .concat2()
    //                     .and_then(move |body| Ok((status, body)))
    //                     .map_err(Error::from)
    //             })
    //             .and_then(|(status, body)| {
    //                 if status.is_success() {
    //                     Ok(body)
    //                 } else {
    //                     Err(Error::from((status, &*body)))
    //                 }
    //             })
    //             .and_then(|_| futures::future::ok(())),
    //     )
    // }
    //
    // fn series_id_images_get(
    //     &self,
    //     id: i64,
    //     accept_language: &str,
    // ) -> Box<dyn Future<Item = crate::models::SeriesImagesCounts, Error = Error<serde_json::Value>>>
    // {
    //     let configuration: &configuration::Configuration<C> = self.configuration.borrow();
    //
    //     let mut auth_headers = HashMap::<String, String>::new();
    //     let auth_query = HashMap::<String, String>::new();
    //     if let Some(ref apikey) = configuration.api_key {
    //         let key = apikey.key.clone();
    //         let val = match apikey.prefix {
    //             Some(ref prefix) => format!("{} {}", prefix, key),
    //             None => key,
    //         };
    //         auth_headers.insert("Authorization".to_owned(), val);
    //     };
    //     let method = hyper::Method::Get;
    //
    //     let query_string = {
    //         let mut query = ::url::form_urlencoded::Serializer::new(String::new());
    //         for (key, val) in &auth_query {
    //             query.append_pair(key, val);
    //         }
    //         query.finish()
    //     };
    //     let uri_str = format!(
    //         "{}/series/{id}/images?{}",
    //         configuration.base_path,
    //         query_string,
    //         id = id
    //     );
    //
    //     // TODO(farcaller): handle error
    //     // if let Err(e) = uri {
    //     //     return Box::new(futures::future::err(e));
    //     // }
    //     let uri: hyper::Uri = uri_str.parse().unwrap();
    //
    //     let mut req = hyper::Request::new(method, uri);
    //
    //     if let Some(ref user_agent) = configuration.user_agent {
    //         req.headers_mut().set(USER_AGENT);
    //     }
    //
    //     {
    //         let headers = req.headers_mut();
    //         headers.set_raw("Accept-Language", accept_language);
    //     }
    //
    //     for (key, val) in auth_headers {
    //         req.headers_mut().set_raw(key, val);
    //     }
    //
    //     // send request
    //     Box::new(
    //         configuration
    //             .client
    //             .request(req)
    //             .map_err(Error::from)
    //             .and_then(|resp| {
    //                 let status = resp.status();
    //                 resp.body()
    //                     .concat2()
    //                     .and_then(move |body| Ok((status, body)))
    //                     .map_err(Error::from)
    //             })
    //             .and_then(|(status, body)| {
    //                 if status.is_success() {
    //                     Ok(body)
    //                 } else {
    //                     Err(Error::from((status, &*body)))
    //                 }
    //             })
    //             .and_then(|body| {
    //                 let parsed: Result<crate::models::SeriesImagesCounts, _> =
    //                     serde_json::from_slice(&body);
    //                 parsed.map_err(Error::from)
    //             }),
    //     )
    // }
    //
    // fn series_id_images_query_get(
    //     &self,
    //     id: i64,
    //     key_type: &str,
    //     resolution: &str,
    //     sub_key: &str,
    //     accept_language: &str,
    // ) -> Box<
    //     dyn Future<Item = crate::models::SeriesImageQueryResults, Error = Error<serde_json::Value>>,
    // > {
    //     let configuration: &configuration::Configuration<C> = self.configuration.borrow();
    //
    //     let mut auth_headers = HashMap::<String, String>::new();
    //     let auth_query = HashMap::<String, String>::new();
    //     if let Some(ref apikey) = configuration.api_key {
    //         let key = apikey.key.clone();
    //         let val = match apikey.prefix {
    //             Some(ref prefix) => format!("{} {}", prefix, key),
    //             None => key,
    //         };
    //         auth_headers.insert("Authorization".to_owned(), val);
    //     };
    //     let method = hyper::Method::Get;
    //
    //     let query_string = {
    //         let mut query = ::url::form_urlencoded::Serializer::new(String::new());
    //         query.append_pair("keyType", &key_type.to_string());
    //         query.append_pair("resolution", &resolution.to_string());
    //         query.append_pair("subKey", &sub_key.to_string());
    //         for (key, val) in &auth_query {
    //             query.append_pair(key, val);
    //         }
    //         query.finish()
    //     };
    //     let uri_str = format!(
    //         "{}/series/{id}/images/query?{}",
    //         configuration.base_path,
    //         query_string,
    //         id = id
    //     );
    //
    //     // TODO(farcaller): handle error
    //     // if let Err(e) = uri {
    //     //     return Box::new(futures::future::err(e));
    //     // }
    //     let uri: hyper::Uri = uri_str.parse().unwrap();
    //
    //     let mut req = hyper::Request::new(method, uri);
    //
    //     if let Some(ref user_agent) = configuration.user_agent {
    //         req.headers_mut().set(USER_AGENT);
    //     }
    //
    //     {
    //         let headers = req.headers_mut();
    //         headers.set_raw("Accept-Language", accept_language);
    //     }
    //
    //     for (key, val) in auth_headers {
    //         req.headers_mut().set_raw(key, val);
    //     }
    //
    //     // send request
    //     Box::new(
    //         configuration
    //             .client
    //             .request(req)
    //             .map_err(Error::from)
    //             .and_then(|resp| {
    //                 let status = resp.status();
    //                 resp.body()
    //                     .concat2()
    //                     .and_then(move |body| Ok((status, body)))
    //                     .map_err(Error::from)
    //             })
    //             .and_then(|(status, body)| {
    //                 if status.is_success() {
    //                     Ok(body)
    //                 } else {
    //                     Err(Error::from((status, &*body)))
    //                 }
    //             })
    //             .and_then(|body| {
    //                 let parsed: Result<crate::models::SeriesImageQueryResults, _> =
    //                     serde_json::from_slice(&body);
    //                 parsed.map_err(Error::from)
    //             }),
    //     )
    // }
}
