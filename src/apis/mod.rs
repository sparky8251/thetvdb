use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T>
where
    T: serde::Deserialize<'de>,
{
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.is_empty() {
            return Error::ApiError(ApiError {
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError {
                code: e.0,
                content: Some(t),
            }),
            Err(e) => Error::from(e),
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

mod authentication_api;
pub use self::authentication_api::{AuthenticationApi, AuthenticationApiClient};
mod episodes_api;
pub use self::episodes_api::{EpisodesApi, EpisodesApiClient};
mod languages_api;
pub use self::languages_api::{LanguagesApi, LanguagesApiClient};
mod search_api;
pub use self::search_api::{SearchApi, SearchApiClient};
mod series_api;
pub use self::series_api::{SeriesApi, SeriesApiClient};
mod updates_api;
pub use self::updates_api::{UpdatesApi, UpdatesApiClient};
// mod users_api;
// pub use self::users_api::{UsersApi, UsersApiClient};

pub mod client;
pub mod configuration;
