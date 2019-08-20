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
pub struct User {
    #[serde(rename = "favoritesDisplaymode")]
    favorites_displaymode: Option<String>,
    language: Option<String>,
    #[serde(rename = "userName")]
    user_name: Option<String>,
}

impl User {
    pub fn set_favorites_displaymode(&mut self, favorites_displaymode: String) {
        self.favorites_displaymode = Some(favorites_displaymode);
    }

    pub fn with_favorites_displaymode(mut self, favorites_displaymode: String) -> User {
        self.favorites_displaymode = Some(favorites_displaymode);
        self
    }

    pub fn favorites_displaymode(&self) -> Option<&String> {
        self.favorites_displaymode.as_ref()
    }

    pub fn reset_favorites_displaymode(&mut self) {
        self.favorites_displaymode = None;
    }

    pub fn set_language(&mut self, language: String) {
        self.language = Some(language);
    }

    pub fn with_language(mut self, language: String) -> User {
        self.language = Some(language);
        self
    }

    pub fn language(&self) -> Option<&String> {
        self.language.as_ref()
    }

    pub fn reset_language(&mut self) {
        self.language = None;
    }

    pub fn set_user_name(&mut self, user_name: String) {
        self.user_name = Some(user_name);
    }

    pub fn with_user_name(mut self, user_name: String) -> User {
        self.user_name = Some(user_name);
        self
    }

    pub fn user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }

    pub fn reset_user_name(&mut self) {
        self.user_name = None;
    }
}