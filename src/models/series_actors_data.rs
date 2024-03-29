/* 
 * TheTVDB API v2
 *
 * API v2 targets v1 functionality with a few minor additions. The API is accessible via https://api.thetvdb.com and provides the following REST endpoints in JSON format.   How to use this API documentation ----------------   You may browse the API routes without authentication, but if you wish to send requests to the API and see response data, then you must authenticate. 1. Obtain a JWT token by `POST`ing  to the `/login` route in the `Authentication` section with your API key and credentials. 1. Paste the JWT token from the response into the \"JWT Token\" field at the top of the page and click the 'Add Token' button.   You will now be able to use the remaining routes to send requests to the API and get a response.   Language Selection ----------------   Language selection is done via the `Accept-Language` header. At the moment, you may only pass one language abbreviation in the header at a time. Valid language abbreviations can be found at the `/languages` route..   Authentication ----------------   Authentication to use the API is similar to the How-to section above. Users must `POST` to the `/login` route with their API key and credentials in the following format in order to obtain a JWT token.  `{\"apikey\":\"APIKEY\",\"username\":\"USERNAME\",\"userkey\":\"USERKEY\"}`  Note that the username and key are ONLY required for the `/user` routes. The user's key is labled `Account Identifier` in the account section of the main site. The token is then used in all subsequent requests by providing it in the `Authorization` header. The header will look like: `Authorization: Bearer <yourJWTtoken>`. Currently, the token expires after 24 hours. You can `GET` the `/refresh_token` route to extend that expiration date.   Versioning ----------------   You may request a different version of the API by including an `Accept` header in your request with the following format: `Accept:application/vnd.thetvdb.v$VERSION`. This documentation automatically uses the version seen at the top and bottom of the page.
 *
 * OpenAPI spec version: 2.2.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesActorsData {
  #[serde(rename = "id")]
  id: Option<i32>,
  #[serde(rename = "image")]
  image: Option<String>,
  #[serde(rename = "imageAdded")]
  image_added: Option<String>,
  #[serde(rename = "imageAuthor")]
  image_author: Option<i32>,
  #[serde(rename = "lastUpdated")]
  last_updated: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "role")]
  role: Option<String>,
  #[serde(rename = "seriesId")]
  series_id: Option<i32>,
  #[serde(rename = "sortOrder")]
  sort_order: Option<i32>
}

impl SeriesActorsData {
  pub fn new() -> SeriesActorsData {
    SeriesActorsData {
      id: None,
      image: None,
      image_added: None,
      image_author: None,
      last_updated: None,
      name: None,
      role: None,
      series_id: None,
      sort_order: None
    }
  }

  pub fn set_id(&mut self, id: i32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i32) -> SeriesActorsData {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i32> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_image(&mut self, image: String) {
    self.image = Some(image);
  }

  pub fn with_image(mut self, image: String) -> SeriesActorsData {
    self.image = Some(image);
    self
  }

  pub fn image(&self) -> Option<&String> {
    self.image.as_ref()
  }

  pub fn reset_image(&mut self) {
    self.image = None;
  }

  pub fn set_image_added(&mut self, image_added: String) {
    self.image_added = Some(image_added);
  }

  pub fn with_image_added(mut self, image_added: String) -> SeriesActorsData {
    self.image_added = Some(image_added);
    self
  }

  pub fn image_added(&self) -> Option<&String> {
    self.image_added.as_ref()
  }

  pub fn reset_image_added(&mut self) {
    self.image_added = None;
  }

  pub fn set_image_author(&mut self, image_author: i32) {
    self.image_author = Some(image_author);
  }

  pub fn with_image_author(mut self, image_author: i32) -> SeriesActorsData {
    self.image_author = Some(image_author);
    self
  }

  pub fn image_author(&self) -> Option<&i32> {
    self.image_author.as_ref()
  }

  pub fn reset_image_author(&mut self) {
    self.image_author = None;
  }

  pub fn set_last_updated(&mut self, last_updated: String) {
    self.last_updated = Some(last_updated);
  }

  pub fn with_last_updated(mut self, last_updated: String) -> SeriesActorsData {
    self.last_updated = Some(last_updated);
    self
  }

  pub fn last_updated(&self) -> Option<&String> {
    self.last_updated.as_ref()
  }

  pub fn reset_last_updated(&mut self) {
    self.last_updated = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> SeriesActorsData {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_role(&mut self, role: String) {
    self.role = Some(role);
  }

  pub fn with_role(mut self, role: String) -> SeriesActorsData {
    self.role = Some(role);
    self
  }

  pub fn role(&self) -> Option<&String> {
    self.role.as_ref()
  }

  pub fn reset_role(&mut self) {
    self.role = None;
  }

  pub fn set_series_id(&mut self, series_id: i32) {
    self.series_id = Some(series_id);
  }

  pub fn with_series_id(mut self, series_id: i32) -> SeriesActorsData {
    self.series_id = Some(series_id);
    self
  }

  pub fn series_id(&self) -> Option<&i32> {
    self.series_id.as_ref()
  }

  pub fn reset_series_id(&mut self) {
    self.series_id = None;
  }

  pub fn set_sort_order(&mut self, sort_order: i32) {
    self.sort_order = Some(sort_order);
  }

  pub fn with_sort_order(mut self, sort_order: i32) -> SeriesActorsData {
    self.sort_order = Some(sort_order);
    self
  }

  pub fn sort_order(&self) -> Option<&i32> {
    self.sort_order.as_ref()
  }

  pub fn reset_sort_order(&mut self) {
    self.sort_order = None;
  }

}



