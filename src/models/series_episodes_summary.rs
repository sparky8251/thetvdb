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
pub struct SeriesEpisodesSummary {
    /// Number of all aired episodes for this series
    #[serde(rename = "airedEpisodes")]
    aired_episodes: Option<String>,
    #[serde(rename = "airedSeasons")]
    aired_seasons: Option<Vec<String>>,
    /// Number of all dvd episodes for this series
    #[serde(rename = "dvdEpisodes")]
    dvd_episodes: Option<String>,
    #[serde(rename = "dvdSeasons")]
    dvd_seasons: Option<Vec<String>>,
}

impl SeriesEpisodesSummary {
    pub fn set_aired_episodes(&mut self, aired_episodes: String) {
        self.aired_episodes = Some(aired_episodes);
    }

    pub fn with_aired_episodes(mut self, aired_episodes: String) -> SeriesEpisodesSummary {
        self.aired_episodes = Some(aired_episodes);
        self
    }

    pub fn aired_episodes(&self) -> Option<&String> {
        self.aired_episodes.as_ref()
    }

    pub fn reset_aired_episodes(&mut self) {
        self.aired_episodes = None;
    }

    pub fn set_aired_seasons(&mut self, aired_seasons: Vec<String>) {
        self.aired_seasons = Some(aired_seasons);
    }

    pub fn with_aired_seasons(mut self, aired_seasons: Vec<String>) -> SeriesEpisodesSummary {
        self.aired_seasons = Some(aired_seasons);
        self
    }

    pub fn aired_seasons(&self) -> Option<&Vec<String>> {
        self.aired_seasons.as_ref()
    }

    pub fn reset_aired_seasons(&mut self) {
        self.aired_seasons = None;
    }

    pub fn set_dvd_episodes(&mut self, dvd_episodes: String) {
        self.dvd_episodes = Some(dvd_episodes);
    }

    pub fn with_dvd_episodes(mut self, dvd_episodes: String) -> SeriesEpisodesSummary {
        self.dvd_episodes = Some(dvd_episodes);
        self
    }

    pub fn dvd_episodes(&self) -> Option<&String> {
        self.dvd_episodes.as_ref()
    }

    pub fn reset_dvd_episodes(&mut self) {
        self.dvd_episodes = None;
    }

    pub fn set_dvd_seasons(&mut self, dvd_seasons: Vec<String>) {
        self.dvd_seasons = Some(dvd_seasons);
    }

    pub fn with_dvd_seasons(mut self, dvd_seasons: Vec<String>) -> SeriesEpisodesSummary {
        self.dvd_seasons = Some(dvd_seasons);
        self
    }

    pub fn dvd_seasons(&self) -> Option<&Vec<String>> {
        self.dvd_seasons.as_ref()
    }

    pub fn reset_dvd_seasons(&mut self) {
        self.dvd_seasons = None;
    }
}