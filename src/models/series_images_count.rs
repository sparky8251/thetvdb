use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SeriesImagesCount {
    fanart: Option<u64>,
    poster: Option<u64>,
    season: Option<u64>,
    seasonwide: Option<u64>,
    series: Option<u64>,
}

impl SeriesImagesCount {
    pub fn set_fanart(&mut self, fanart: u64) {
        self.fanart = Some(fanart);
    }

    pub fn with_fanart(mut self, fanart: u64) -> SeriesImagesCount {
        self.fanart = Some(fanart);
        self
    }

    pub fn fanart(&self) -> Option<&u64> {
        self.fanart.as_ref()
    }

    pub fn reset_fanart(&mut self) {
        self.fanart = None;
    }

    pub fn set_poster(&mut self, poster: u64) {
        self.poster = Some(poster);
    }

    pub fn with_poster(mut self, poster: u64) -> SeriesImagesCount {
        self.poster = Some(poster);
        self
    }

    pub fn poster(&self) -> Option<&u64> {
        self.poster.as_ref()
    }

    pub fn reset_poster(&mut self) {
        self.poster = None;
    }

    pub fn set_season(&mut self, season: u64) {
        self.season = Some(season);
    }

    pub fn with_season(mut self, season: u64) -> SeriesImagesCount {
        self.season = Some(season);
        self
    }

    pub fn season(&self) -> Option<&u64> {
        self.season.as_ref()
    }

    pub fn reset_season(&mut self) {
        self.season = None;
    }

    pub fn set_seasonwide(&mut self, seasonwide: u64) {
        self.seasonwide = Some(seasonwide);
    }

    pub fn with_seasonwide(mut self, seasonwide: u64) -> SeriesImagesCount {
        self.seasonwide = Some(seasonwide);
        self
    }

    pub fn seasonwide(&self) -> Option<&u64> {
        self.seasonwide.as_ref()
    }

    pub fn reset_seasonwide(&mut self) {
        self.seasonwide = None;
    }

    pub fn set_series(&mut self, series: u64) {
        self.series = Some(series);
    }

    pub fn with_series(mut self, series: u64) -> SeriesImagesCount {
        self.series = Some(series);
        self
    }

    pub fn series(&self) -> Option<&u64> {
        self.series.as_ref()
    }

    pub fn reset_series(&mut self) {
        self.series = None;
    }
}
