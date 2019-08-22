// TODO: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=1d341fceb240b0ea5d247e04deb57cc8
// Impl the above fancy type system to prevent duplicate insertions

use std::collections::HashSet;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct SeriesIdEpisodesQueryParamsEmpty {}

#[derive(Debug)]
pub struct SeriesIdEpisodesQueryParams {
    values: HashSet<SeriesIdEpisodesQueryParamValue>,
}

#[derive(Debug)]
pub enum SeriesIdEpisodesQueryParamValue {
    AbsoluteNumber(u64),
    AiredSeason(u64),
    AiredEpisode(u64),
    DvdSeason(u64),
    DvdEpisode(u64),
    ImdbId(String),
    Page(u64),
}

impl SeriesIdEpisodesQueryParamsEmpty {
    pub fn insert(self, value: SeriesIdEpisodesQueryParamValue) -> SeriesIdEpisodesQueryParams {
        let mut values = HashSet::new();
        values.insert(value);
        SeriesIdEpisodesQueryParams { values }
    }
}

impl SeriesIdEpisodesQueryParams {
    pub fn builder() -> SeriesIdEpisodesQueryParamsEmpty {
        SeriesIdEpisodesQueryParamsEmpty {}
    }

    pub fn insert(&mut self, value: SeriesIdEpisodesQueryParamValue) {
        self.values.insert(value);
    }
}

impl Display for SeriesIdEpisodesQueryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use SeriesIdEpisodesQueryParamValue::*;

        if self.values.is_empty() {
            return Err(std::fmt::Error);
        }

        let query = self
            .values
            .iter()
            .map(|v| match v {
                AbsoluteNumber(x) => format!("absolute_number={}", x),
                AiredSeason(x) => format!("aired_season={}", x),
                AiredEpisode(x) => format!("aired_episode={}", x),
                DvdSeason(x) => format!("dvd_season={}", x),
                DvdEpisode(x) => format!("dvd_episode={}", x),
                ImdbId(x) => format!("imdb_id={}", x),
                Page(x) => format!("page={}", x),
            })
            .collect::<Vec<_>>()
            .join("&");

        write!(f, "?{}", query)
    }
}

impl Display for SeriesIdEpisodesQueryParamValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AbsoluteNumber(v) => write!(f, "{}", v),
            Self::AiredSeason(v) => write!(f, "{}", v),
            Self::AiredEpisode(v) => write!(f, "{}", v),
            Self::DvdSeason(v) => write!(f, "{}", v),
            Self::DvdEpisode(v) => write!(f, "{}", v),
            Self::ImdbId(v) => write!(f, "{}", v),
            Self::Page(v) => write!(f, "{}", v),
        }
    }
}

impl Hash for SeriesIdEpisodesQueryParamValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl PartialEq for SeriesIdEpisodesQueryParamValue {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
impl Eq for SeriesIdEpisodesQueryParamValue {}
