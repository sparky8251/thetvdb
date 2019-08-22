use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BasicEpisode {
    #[serde(rename = "absoluteNumber")]
    absolute_number: Option<u64>,
    #[serde(rename = "airedEpisodeNumber")]
    aired_episode_number: Option<u64>,
    #[serde(rename = "airedSeason")]
    aired_season: Option<u64>,
    #[serde(rename = "dvdEpisodeNumber")]
    dvd_episode_number: Option<u64>,
    #[serde(rename = "dvdSeason")]
    dvd_season: Option<u64>,
    #[serde(rename = "episodeName")]
    episode_name: Option<String>,
    #[serde(rename = "firstAired")]
    first_aired: Option<String>,
    id: Option<u64>,
    language: Option<crate::models::EpisodeLanguageInfo>,
    #[serde(rename = "lastUpdated")]
    last_updated: Option<u64>,
    #[serde(rename = "overview")]
    overview: Option<String>,
}

impl BasicEpisode {
    pub fn set_absolute_number(&mut self, absolute_number: u64) {
        self.absolute_number = Some(absolute_number);
    }

    pub fn with_absolute_number(mut self, absolute_number: u64) -> BasicEpisode {
        self.absolute_number = Some(absolute_number);
        self
    }

    pub fn absolute_number(&self) -> Option<&u64> {
        self.absolute_number.as_ref()
    }

    pub fn reset_absolute_number(&mut self) {
        self.absolute_number = None;
    }

    pub fn set_aired_episode_number(&mut self, aired_episode_number: u64) {
        self.aired_episode_number = Some(aired_episode_number);
    }

    pub fn with_aired_episode_number(mut self, aired_episode_number: u64) -> BasicEpisode {
        self.aired_episode_number = Some(aired_episode_number);
        self
    }

    pub fn aired_episode_number(&self) -> Option<&u64> {
        self.aired_episode_number.as_ref()
    }

    pub fn reset_aired_episode_number(&mut self) {
        self.aired_episode_number = None;
    }

    pub fn set_aired_season(&mut self, aired_season: u64) {
        self.aired_season = Some(aired_season);
    }

    pub fn with_aired_season(mut self, aired_season: u64) -> BasicEpisode {
        self.aired_season = Some(aired_season);
        self
    }

    pub fn aired_season(&self) -> Option<&u64> {
        self.aired_season.as_ref()
    }

    pub fn reset_aired_season(&mut self) {
        self.aired_season = None;
    }

    pub fn set_dvd_episode_number(&mut self, dvd_episode_number: u64) {
        self.dvd_episode_number = Some(dvd_episode_number);
    }

    pub fn with_dvd_episode_number(mut self, dvd_episode_number: u64) -> BasicEpisode {
        self.dvd_episode_number = Some(dvd_episode_number);
        self
    }

    pub fn dvd_episode_number(&self) -> Option<&u64> {
        self.dvd_episode_number.as_ref()
    }

    pub fn reset_dvd_episode_number(&mut self) {
        self.dvd_episode_number = None;
    }

    pub fn set_dvd_season(&mut self, dvd_season: u64) {
        self.dvd_season = Some(dvd_season);
    }

    pub fn with_dvd_season(mut self, dvd_season: u64) -> BasicEpisode {
        self.dvd_season = Some(dvd_season);
        self
    }

    pub fn dvd_season(&self) -> Option<&u64> {
        self.dvd_season.as_ref()
    }

    pub fn reset_dvd_season(&mut self) {
        self.dvd_season = None;
    }

    pub fn set_episode_name(&mut self, episode_name: String) {
        self.episode_name = Some(episode_name);
    }

    pub fn with_episode_name(mut self, episode_name: String) -> BasicEpisode {
        self.episode_name = Some(episode_name);
        self
    }

    pub fn episode_name(&self) -> Option<&String> {
        self.episode_name.as_ref()
    }

    pub fn reset_episode_name(&mut self) {
        self.episode_name = None;
    }

    pub fn set_first_aired(&mut self, first_aired: String) {
        self.first_aired = Some(first_aired);
    }

    pub fn with_first_aired(mut self, first_aired: String) -> BasicEpisode {
        self.first_aired = Some(first_aired);
        self
    }

    pub fn first_aired(&self) -> Option<&String> {
        self.first_aired.as_ref()
    }

    pub fn reset_first_aired(&mut self) {
        self.first_aired = None;
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: u64) -> BasicEpisode {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&u64> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_language(&mut self, language: crate::models::EpisodeLanguageInfo) {
        self.language = Some(language);
    }

    pub fn with_language(mut self, language: crate::models::EpisodeLanguageInfo) -> BasicEpisode {
        self.language = Some(language);
        self
    }

    pub fn language(&self) -> Option<&crate::models::EpisodeLanguageInfo> {
        self.language.as_ref()
    }

    pub fn reset_language(&mut self) {
        self.language = None;
    }

    pub fn set_last_updated(&mut self, last_updated: u64) {
        self.last_updated = Some(last_updated);
    }

    pub fn with_last_updated(mut self, last_updated: u64) -> BasicEpisode {
        self.last_updated = Some(last_updated);
        self
    }

    pub fn last_updated(&self) -> Option<&u64> {
        self.last_updated.as_ref()
    }

    pub fn reset_last_updated(&mut self) {
        self.last_updated = None;
    }

    pub fn set_overview(&mut self, overview: String) {
        self.overview = Some(overview);
    }

    pub fn with_overview(mut self, overview: String) -> BasicEpisode {
        self.overview = Some(overview);
        self
    }

    pub fn overview(&self) -> Option<&String> {
        self.overview.as_ref()
    }

    pub fn reset_overview(&mut self) {
        self.overview = None;
    }
}
