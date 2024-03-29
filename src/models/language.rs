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
pub struct Language {
  #[serde(rename = "abbreviation")]
  abbreviation: Option<String>,
  #[serde(rename = "englishName")]
  english_name: Option<String>,
  #[serde(rename = "id")]
  id: Option<i32>,
  #[serde(rename = "name")]
  name: Option<String>
}

impl Language {
  pub fn new() -> Language {
    Language {
      abbreviation: None,
      english_name: None,
      id: None,
      name: None
    }
  }

  pub fn set_abbreviation(&mut self, abbreviation: String) {
    self.abbreviation = Some(abbreviation);
  }

  pub fn with_abbreviation(mut self, abbreviation: String) -> Language {
    self.abbreviation = Some(abbreviation);
    self
  }

  pub fn abbreviation(&self) -> Option<&String> {
    self.abbreviation.as_ref()
  }

  pub fn reset_abbreviation(&mut self) {
    self.abbreviation = None;
  }

  pub fn set_english_name(&mut self, english_name: String) {
    self.english_name = Some(english_name);
  }

  pub fn with_english_name(mut self, english_name: String) -> Language {
    self.english_name = Some(english_name);
    self
  }

  pub fn english_name(&self) -> Option<&String> {
    self.english_name.as_ref()
  }

  pub fn reset_english_name(&mut self) {
    self.english_name = None;
  }

  pub fn set_id(&mut self, id: i32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i32) -> Language {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i32> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> Language {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

}



