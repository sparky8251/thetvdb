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
