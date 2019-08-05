use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  authentication_api: Box<crate::apis::AuthenticationApi>,
  episodes_api: Box<crate::apis::EpisodesApi>,
  languages_api: Box<crate::apis::LanguagesApi>,
  search_api: Box<crate::apis::SearchApi>,
  series_api: Box<crate::apis::SeriesApi>,
  updates_api: Box<crate::apis::UpdatesApi>,
  users_api: Box<crate::apis::UsersApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      authentication_api: Box::new(crate::apis::AuthenticationApiClient::new(rc.clone())),
      episodes_api: Box::new(crate::apis::EpisodesApiClient::new(rc.clone())),
      languages_api: Box::new(crate::apis::LanguagesApiClient::new(rc.clone())),
      search_api: Box::new(crate::apis::SearchApiClient::new(rc.clone())),
      series_api: Box::new(crate::apis::SeriesApiClient::new(rc.clone())),
      updates_api: Box::new(crate::apis::UpdatesApiClient::new(rc.clone())),
      users_api: Box::new(crate::apis::UsersApiClient::new(rc.clone())),
    }
  }

  pub fn authentication_api(&self) -> &crate::apis::AuthenticationApi{
    self.authentication_api.as_ref()
  }

  pub fn episodes_api(&self) -> &crate::apis::EpisodesApi{
    self.episodes_api.as_ref()
  }

  pub fn languages_api(&self) -> &crate::apis::LanguagesApi{
    self.languages_api.as_ref()
  }

  pub fn search_api(&self) -> &crate::apis::SearchApi{
    self.search_api.as_ref()
  }

  pub fn series_api(&self) -> &crate::apis::SeriesApi{
    self.series_api.as_ref()
  }

  pub fn updates_api(&self) -> &crate::apis::UpdatesApi{
    self.updates_api.as_ref()
  }

  pub fn users_api(&self) -> &crate::apis::UsersApi{
    self.users_api.as_ref()
  }


}
