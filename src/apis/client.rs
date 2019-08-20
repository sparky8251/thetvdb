use std::rc::Rc;

use super::configuration::Configuration;
use hyper;

pub struct APIClient<C: hyper::client::connect::Connect> {
    #[allow(dead_code)]
    configuration: Rc<Configuration<C>>,
    authentication_api: Box<dyn crate::apis::AuthenticationApi>,
    episodes_api: Box<dyn crate::apis::EpisodesApi>,
    languages_api: Box<dyn crate::apis::LanguagesApi>,
    // search_api: Box<dyn crate::apis::SearchApi>,
    // series_api: Box<dyn crate::apis::SeriesApi>,
    // updates_api: Box<dyn crate::apis::UpdatesApi>,
    // users_api: Box<dyn crate::apis::UsersApi>,
}

impl<C: 'static + hyper::client::connect::Connect> APIClient<C> {
    pub fn new(configuration: Configuration<C>) -> APIClient<C> {
        let rc = Rc::new(configuration);

        APIClient {
            configuration: rc.clone(),
            authentication_api: Box::new(crate::apis::AuthenticationApiClient::new(rc.clone())),
            episodes_api: Box::new(crate::apis::EpisodesApiClient::new(rc.clone())),
            languages_api: Box::new(crate::apis::LanguagesApiClient::new(rc.clone())),
            // search_api: Box::new(crate::apis::SearchApiClient::new(rc.clone())),
            // series_api: Box::new(crate::apis::SeriesApiClient::new(rc.clone())),
            // updates_api: Box::new(crate::apis::UpdatesApiClient::new(rc.clone())),
            // users_api: Box::new(crate::apis::UsersApiClient::new(rc.clone())),
        }
    }

    pub fn authentication_api(&self) -> &dyn crate::apis::AuthenticationApi {
        self.authentication_api.as_ref()
    }

    pub fn episodes_api(&self) -> &dyn crate::apis::EpisodesApi {
        self.episodes_api.as_ref()
    }

    pub fn languages_api(&self) -> &dyn crate::apis::LanguagesApi {
        self.languages_api.as_ref()
    }

    // pub fn search_api(&self) -> &dyn crate::apis::SearchApi {
    //     self.search_api.as_ref()
    // }
    //
    // pub fn series_api(&self) -> &dyn crate::apis::SeriesApi {
    //     self.series_api.as_ref()
    // }
    //
    // pub fn updates_api(&self) -> &dyn crate::apis::UpdatesApi {
    //     self.updates_api.as_ref()
    // }
    //
    // pub fn users_api(&self) -> &dyn crate::apis::UsersApi {
    //     self.users_api.as_ref()
    // }
}
