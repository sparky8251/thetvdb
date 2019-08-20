use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EpisodeLanguageInfo {
    #[serde(rename = "episodeName")]
    episode_name: Option<String>,
    overview: Option<String>,
}

impl EpisodeLanguageInfo {
    pub fn set_episode_name(&mut self, episode_name: String) {
        self.episode_name = Some(episode_name);
    }

    pub fn with_episode_name(mut self, episode_name: String) -> EpisodeLanguageInfo {
        self.episode_name = Some(episode_name);
        self
    }

    pub fn episode_name(&self) -> Option<&String> {
        self.episode_name.as_ref()
    }

    pub fn reset_episode_name(&mut self) {
        self.episode_name = None;
    }

    pub fn set_overview(&mut self, overview: String) {
        self.overview = Some(overview);
    }

    pub fn with_overview(mut self, overview: String) -> EpisodeLanguageInfo {
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
