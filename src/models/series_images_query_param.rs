/*
 * TheTVDB API v2
 *
 * API v2 targets v1 functionality with a few minor additions. The API is accessible via https://api.thetvdb.com and provides the following REST endpoints in JSON format.   How to use this API documentation ----------------   You may browse the API routes without authentication, but if you wish to send requests to the API and see response data, then you must authenticate. 1. Obtain a JWT token by `POST`ing  to the `/login` route in the `Authentication` section with your API key and credentials. 1. Paste the JWT token from the response into the \"JWT Token\" field at the top of the page and click the 'Add Token' button.   You will now be able to use the remaining routes to send requests to the API and get a response.   Language Selection ----------------   Language selection is done via the `Accept-Language` header. At the moment, you may only pass one language abbreviation in the header at a time. Valid language abbreviations can be found at the `/languages` route..   Authentication ----------------   Authentication to use the API is similar to the How-to section above. Users must `POST` to the `/login` route with their API key and credentials in the following format in order to obtain a JWT token.  `{\"apikey\":\"APIKEY\",\"username\":\"USERNAME\",\"userkey\":\"USERKEY\"}`  Note that the username and key are ONLY required for the `/user` routes. The user's key is labled `Account Identifier` in the account section of the main site. The token is then used in all subsequent requests by providing it in the `Authorization` header. The header will look like: `Authorization: Bearer <yourJWTtoken>`. Currently, the token expires after 24 hours. You can `GET` the `/refresh_token` route to extend that expiration date.   Versioning ----------------   You may request a different version of the API by including an `Accept` header in your request with the following format: `Accept:application/vnd.thetvdb.v$VERSION`. This documentation automatically uses the version seen at the top and bottom of the page.
 *
 * OpenAPI spec version: 2.2.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SeriesImagesQueryParam {
    #[serde(rename = "keyType")]
    key_type: Option<String>,
    #[serde(rename = "languageId")]
    language_id: Option<String>,
    resolution: Option<Vec<String>>,
    sub_key: Option<Vec<String>>,
}

impl SeriesImagesQueryParam {
    pub fn set_key_type(&mut self, key_type: String) {
        self.key_type = Some(key_type);
    }

    pub fn with_key_type(mut self, key_type: String) -> SeriesImagesQueryParam {
        self.key_type = Some(key_type);
        self
    }

    pub fn key_type(&self) -> Option<&String> {
        self.key_type.as_ref()
    }

    pub fn reset_key_type(&mut self) {
        self.key_type = None;
    }

    pub fn set_language_id(&mut self, language_id: String) {
        self.language_id = Some(language_id);
    }

    pub fn with_language_id(mut self, language_id: String) -> SeriesImagesQueryParam {
        self.language_id = Some(language_id);
        self
    }

    pub fn language_id(&self) -> Option<&String> {
        self.language_id.as_ref()
    }

    pub fn reset_language_id(&mut self) {
        self.language_id = None;
    }

    pub fn set_resolution(&mut self, resolution: Vec<String>) {
        self.resolution = Some(resolution);
    }

    pub fn with_resolution(mut self, resolution: Vec<String>) -> SeriesImagesQueryParam {
        self.resolution = Some(resolution);
        self
    }

    pub fn resolution(&self) -> Option<&Vec<String>> {
        self.resolution.as_ref()
    }

    pub fn reset_resolution(&mut self) {
        self.resolution = None;
    }

    pub fn set_sub_key(&mut self, sub_key: Vec<String>) {
        self.sub_key = Some(sub_key);
    }

    pub fn with_sub_key(mut self, sub_key: Vec<String>) -> SeriesImagesQueryParam {
        self.sub_key = Some(sub_key);
        self
    }

    pub fn sub_key(&self) -> Option<&Vec<String>> {
        self.sub_key.as_ref()
    }

    pub fn reset_sub_key(&mut self) {
        self.sub_key = None;
    }
}