use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

use futures;
use futures::{Future, Stream};
use hyper;
use serde_json;

use hyper::header::USER_AGENT;

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
        page: &str,
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
        page: &str,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::SeriesEpisodesQuery, Error = Error<serde_json::Value>>>;
    fn series_id_episodes_query_params_get(
        &self,
        id: i64,
    ) -> Box<
        dyn Future<
            Item = crate::models::SeriesEpisodesQueryParams,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn series_id_episodes_summary_get(
        &self,
        id: i64,
    ) -> Box<
        dyn Future<Item = crate::models::SeriesEpisodesSummary, Error = Error<serde_json::Value>>,
    >;
    fn series_id_filter_get(
        &self,
        id: i64,
        keys: &str,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::SeriesData, Error = Error<serde_json::Value>>>;
    fn series_id_filter_params_get(
        &self,
        id: i64,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::FilterKeys, Error = Error<serde_json::Value>>>;
    fn series_id_get(
        &self,
        id: i64,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::SeriesData, Error = Error<serde_json::Value>>>;
    fn series_id_head(
        &self,
        id: i64,
        accept_language: &str,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn series_id_images_get(
        &self,
        id: i64,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::SeriesImagesCounts, Error = Error<serde_json::Value>>>;
    fn series_id_images_query_get(
        &self,
        id: i64,
        key_type: &str,
        resolution: &str,
        sub_key: &str,
        accept_language: &str,
    ) -> Box<
        dyn Future<Item = crate::models::SeriesImageQueryResults, Error = Error<serde_json::Value>>,
    >;
    fn series_id_images_query_params_get(
        &self,
        id: i64,
        accept_language: &str,
    ) -> Box<
        dyn Future<Item = crate::models::SeriesImagesQueryParams, Error = Error<serde_json::Value>>,
    >;
}

impl<C: hyper::client::connect::Connect> SeriesApi for SeriesApiClient<C> {
    fn series_id_actors_get(
        &self,
        id: i64,
    ) -> Box<dyn Future<Item = crate::models::SeriesActors, Error = Error<serde_json::Value>>> {
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
            "{}/series/{id}/actors?{}",
            configuration.base_path,
            query_string,
            id = id
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
                    let parsed: Result<crate::models::SeriesActors, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn series_id_episodes_get(
        &self,
        id: i64,
        page: &str,
    ) -> Box<dyn Future<Item = crate::models::SeriesEpisodes, Error = Error<serde_json::Value>>>
    {
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
            query.append_pair("page", &page.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/series/{id}/episodes?{}",
            configuration.base_path,
            query_string,
            id = id
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
                    let parsed: Result<crate::models::SeriesEpisodes, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn series_id_episodes_query_get(
        &self,
        id: i64,
        absolute_number: &str,
        aired_season: &str,
        aired_episode: &str,
        dvd_season: &str,
        dvd_episode: &str,
        imdb_id: &str,
        page: &str,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::SeriesEpisodesQuery, Error = Error<serde_json::Value>>>
    {
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
            query.append_pair("absoluteNumber", &absolute_number.to_string());
            query.append_pair("airedSeason", &aired_season.to_string());
            query.append_pair("airedEpisode", &aired_episode.to_string());
            query.append_pair("dvdSeason", &dvd_season.to_string());
            query.append_pair("dvdEpisode", &dvd_episode.to_string());
            query.append_pair("imdbId", &imdb_id.to_string());
            query.append_pair("page", &page.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/series/{id}/episodes/query?{}",
            configuration.base_path,
            query_string,
            id = id
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
                    let parsed: Result<crate::models::SeriesEpisodesQuery, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn series_id_episodes_query_params_get(
        &self,
        id: i64,
    ) -> Box<
        dyn Future<
            Item = crate::models::SeriesEpisodesQueryParams,
            Error = Error<serde_json::Value>,
        >,
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
            "{}/series/{id}/episodes/query/params?{}",
            configuration.base_path,
            query_string,
            id = id
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
                    let parsed: Result<crate::models::SeriesEpisodesQueryParams, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn series_id_episodes_summary_get(
        &self,
        id: i64,
    ) -> Box<
        dyn Future<Item = crate::models::SeriesEpisodesSummary, Error = Error<serde_json::Value>>,
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
            "{}/series/{id}/episodes/summary?{}",
            configuration.base_path,
            query_string,
            id = id
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
                    let parsed: Result<crate::models::SeriesEpisodesSummary, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn series_id_filter_get(
        &self,
        id: i64,
        keys: &str,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::SeriesData, Error = Error<serde_json::Value>>> {
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
            query.append_pair("keys", &keys.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/series/{id}/filter?{}",
            configuration.base_path,
            query_string,
            id = id
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
                    let parsed: Result<crate::models::SeriesData, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn series_id_filter_params_get(
        &self,
        id: i64,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::FilterKeys, Error = Error<serde_json::Value>>> {
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
            "{}/series/{id}/filter/params?{}",
            configuration.base_path,
            query_string,
            id = id
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
                    let parsed: Result<crate::models::FilterKeys, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn series_id_get(
        &self,
        id: i64,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::SeriesData, Error = Error<serde_json::Value>>> {
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
            "{}/series/{id}?{}",
            configuration.base_path,
            query_string,
            id = id
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
                    let parsed: Result<crate::models::SeriesData, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn series_id_head(
        &self,
        id: i64,
        accept_language: &str,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
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
        let method = hyper::Method::Head;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/series/{id}?{}",
            configuration.base_path,
            query_string,
            id = id
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
                .and_then(|_| futures::future::ok(())),
        )
    }

    fn series_id_images_get(
        &self,
        id: i64,
        accept_language: &str,
    ) -> Box<dyn Future<Item = crate::models::SeriesImagesCounts, Error = Error<serde_json::Value>>>
    {
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
            "{}/series/{id}/images?{}",
            configuration.base_path,
            query_string,
            id = id
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
                    let parsed: Result<crate::models::SeriesImagesCounts, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn series_id_images_query_get(
        &self,
        id: i64,
        key_type: &str,
        resolution: &str,
        sub_key: &str,
        accept_language: &str,
    ) -> Box<
        dyn Future<Item = crate::models::SeriesImageQueryResults, Error = Error<serde_json::Value>>,
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
            query.append_pair("keyType", &key_type.to_string());
            query.append_pair("resolution", &resolution.to_string());
            query.append_pair("subKey", &sub_key.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!(
            "{}/series/{id}/images/query?{}",
            configuration.base_path,
            query_string,
            id = id
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
                    let parsed: Result<crate::models::SeriesImageQueryResults, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn series_id_images_query_params_get(
        &self,
        id: i64,
        accept_language: &str,
    ) -> Box<
        dyn Future<Item = crate::models::SeriesImagesQueryParams, Error = Error<serde_json::Value>>,
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
            "{}/series/{id}/images/query/params?{}",
            configuration.base_path,
            query_string,
            id = id
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
                    let parsed: Result<crate::models::SeriesImagesQueryParams, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }
}
