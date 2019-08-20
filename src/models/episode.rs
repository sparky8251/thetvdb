use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Episode {
    #[serde(rename = "absoluteNumber")]
    absolute_number: Option<i64>,
    #[serde(rename = "airedEpisodeNumber")]
    aired_episode_number: Option<i64>,
    #[serde(rename = "airedSeason")]
    aired_season: Option<i64>,
    #[serde(rename = "airsAfterSeason")]
    airs_after_season: Option<i64>,
    #[serde(rename = "airsBeforeEpisode")]
    airs_before_episode: Option<i64>,
    #[serde(rename = "airsBeforeSeason")]
    airs_before_season: Option<i64>,
    director: Option<String>,
    directors: Option<Vec<String>>,
    #[serde(rename = "dvdChapter")]
    dvd_chapter: Option<f64>,
    #[serde(rename = "dvdDiscid")]
    dvd_discid: Option<String>,
    #[serde(rename = "dvdEpisodeNumber")]
    dvd_episode_number: Option<f64>,
    #[serde(rename = "dvdSeason")]
    dvd_season: Option<i64>,
    #[serde(rename = "episodeName")]
    episode_name: Option<String>,
    filename: Option<String>,
    #[serde(rename = "firstAired")]
    first_aired: Option<String>,
    #[serde(rename = "guestStars")]
    guest_stars: Option<Vec<String>>,
    id: Option<i64>,
    #[serde(rename = "imdbId")]
    imdb_id: Option<String>,
    #[serde(rename = "lastUpdated")]
    last_updated: Option<i64>,
    #[serde(rename = "lastUpdatedBy")]
    last_updated_by: Option<i64>,
    overview: Option<String>,
    #[serde(rename = "productionCode")]
    production_code: Option<String>,
    #[serde(rename = "seriesId")]
    series_id: Option<i64>,
    #[serde(rename = "showUrl")]
    show_url: Option<String>,
    #[serde(rename = "siteRating")]
    site_rating: Option<f64>,
    #[serde(rename = "siteRatingCount")]
    site_rating_count: Option<i64>,
    #[serde(rename = "thumbAdded")]
    thumb_added: Option<String>,
    #[serde(rename = "thumbAuthor")]
    thumb_author: Option<i64>,
    #[serde(rename = "thumbHeight")]
    thumb_height: Option<String>,
    #[serde(rename = "thumbWidth")]
    thumb_width: Option<String>,
    writers: Option<Vec<String>>,
}

impl Episode {
    pub fn set_absolute_number(&mut self, absolute_number: i64) {
        self.absolute_number = Some(absolute_number);
    }

    pub fn with_absolute_number(mut self, absolute_number: i64) -> Episode {
        self.absolute_number = Some(absolute_number);
        self
    }

    pub fn absolute_number(&self) -> Option<&i64> {
        self.absolute_number.as_ref()
    }

    pub fn reset_absolute_number(&mut self) {
        self.absolute_number = None;
    }

    pub fn set_aired_episode_number(&mut self, aired_episode_number: i64) {
        self.aired_episode_number = Some(aired_episode_number);
    }

    pub fn with_aired_episode_number(mut self, aired_episode_number: i64) -> Episode {
        self.aired_episode_number = Some(aired_episode_number);
        self
    }

    pub fn aired_episode_number(&self) -> Option<&i64> {
        self.aired_episode_number.as_ref()
    }

    pub fn reset_aired_episode_number(&mut self) {
        self.aired_episode_number = None;
    }

    pub fn set_aired_season(&mut self, aired_season: i64) {
        self.aired_season = Some(aired_season);
    }

    pub fn with_aired_season(mut self, aired_season: i64) -> Episode {
        self.aired_season = Some(aired_season);
        self
    }

    pub fn aired_season(&self) -> Option<&i64> {
        self.aired_season.as_ref()
    }

    pub fn reset_aired_season(&mut self) {
        self.aired_season = None;
    }

    pub fn set_airs_after_season(&mut self, airs_after_season: i64) {
        self.airs_after_season = Some(airs_after_season);
    }

    pub fn with_airs_after_season(mut self, airs_after_season: i64) -> Episode {
        self.airs_after_season = Some(airs_after_season);
        self
    }

    pub fn airs_after_season(&self) -> Option<&i64> {
        self.airs_after_season.as_ref()
    }

    pub fn reset_airs_after_season(&mut self) {
        self.airs_after_season = None;
    }

    pub fn set_airs_before_episode(&mut self, airs_before_episode: i64) {
        self.airs_before_episode = Some(airs_before_episode);
    }

    pub fn with_airs_before_episode(mut self, airs_before_episode: i64) -> Episode {
        self.airs_before_episode = Some(airs_before_episode);
        self
    }

    pub fn airs_before_episode(&self) -> Option<&i64> {
        self.airs_before_episode.as_ref()
    }

    pub fn reset_airs_before_episode(&mut self) {
        self.airs_before_episode = None;
    }

    pub fn set_airs_before_season(&mut self, airs_before_season: i64) {
        self.airs_before_season = Some(airs_before_season);
    }

    pub fn with_airs_before_season(mut self, airs_before_season: i64) -> Episode {
        self.airs_before_season = Some(airs_before_season);
        self
    }

    pub fn airs_before_season(&self) -> Option<&i64> {
        self.airs_before_season.as_ref()
    }

    pub fn reset_airs_before_season(&mut self) {
        self.airs_before_season = None;
    }

    pub fn set_director(&mut self, director: String) {
        self.director = Some(director);
    }

    pub fn with_director(mut self, director: String) -> Episode {
        self.director = Some(director);
        self
    }

    pub fn director(&self) -> Option<&String> {
        self.director.as_ref()
    }

    pub fn reset_director(&mut self) {
        self.director = None;
    }

    pub fn set_directors(&mut self, directors: Vec<String>) {
        self.directors = Some(directors);
    }

    pub fn with_directors(mut self, directors: Vec<String>) -> Episode {
        self.directors = Some(directors);
        self
    }

    pub fn directors(&self) -> Option<&Vec<String>> {
        self.directors.as_ref()
    }

    pub fn reset_directors(&mut self) {
        self.directors = None;
    }

    pub fn set_dvd_chapter(&mut self, dvd_chapter: f64) {
        self.dvd_chapter = Some(dvd_chapter);
    }

    pub fn with_dvd_chapter(mut self, dvd_chapter: f64) -> Episode {
        self.dvd_chapter = Some(dvd_chapter);
        self
    }

    pub fn dvd_chapter(&self) -> Option<&f64> {
        self.dvd_chapter.as_ref()
    }

    pub fn reset_dvd_chapter(&mut self) {
        self.dvd_chapter = None;
    }

    pub fn set_dvd_discid(&mut self, dvd_discid: String) {
        self.dvd_discid = Some(dvd_discid);
    }

    pub fn with_dvd_discid(mut self, dvd_discid: String) -> Episode {
        self.dvd_discid = Some(dvd_discid);
        self
    }

    pub fn dvd_discid(&self) -> Option<&String> {
        self.dvd_discid.as_ref()
    }

    pub fn reset_dvd_discid(&mut self) {
        self.dvd_discid = None;
    }

    pub fn set_dvd_episode_number(&mut self, dvd_episode_number: f64) {
        self.dvd_episode_number = Some(dvd_episode_number);
    }

    pub fn with_dvd_episode_number(mut self, dvd_episode_number: f64) -> Episode {
        self.dvd_episode_number = Some(dvd_episode_number);
        self
    }

    pub fn dvd_episode_number(&self) -> Option<&f64> {
        self.dvd_episode_number.as_ref()
    }

    pub fn reset_dvd_episode_number(&mut self) {
        self.dvd_episode_number = None;
    }

    pub fn set_dvd_season(&mut self, dvd_season: i64) {
        self.dvd_season = Some(dvd_season);
    }

    pub fn with_dvd_season(mut self, dvd_season: i64) -> Episode {
        self.dvd_season = Some(dvd_season);
        self
    }

    pub fn dvd_season(&self) -> Option<&i64> {
        self.dvd_season.as_ref()
    }

    pub fn reset_dvd_season(&mut self) {
        self.dvd_season = None;
    }

    pub fn set_episode_name(&mut self, episode_name: String) {
        self.episode_name = Some(episode_name);
    }

    pub fn with_episode_name(mut self, episode_name: String) -> Episode {
        self.episode_name = Some(episode_name);
        self
    }

    pub fn episode_name(&self) -> Option<&String> {
        self.episode_name.as_ref()
    }

    pub fn reset_episode_name(&mut self) {
        self.episode_name = None;
    }

    pub fn set_filename(&mut self, filename: String) {
        self.filename = Some(filename);
    }

    pub fn with_filename(mut self, filename: String) -> Episode {
        self.filename = Some(filename);
        self
    }

    pub fn filename(&self) -> Option<&String> {
        self.filename.as_ref()
    }

    pub fn reset_filename(&mut self) {
        self.filename = None;
    }

    pub fn set_first_aired(&mut self, first_aired: String) {
        self.first_aired = Some(first_aired);
    }

    pub fn with_first_aired(mut self, first_aired: String) -> Episode {
        self.first_aired = Some(first_aired);
        self
    }

    pub fn first_aired(&self) -> Option<&String> {
        self.first_aired.as_ref()
    }

    pub fn reset_first_aired(&mut self) {
        self.first_aired = None;
    }

    pub fn set_guest_stars(&mut self, guest_stars: Vec<String>) {
        self.guest_stars = Some(guest_stars);
    }

    pub fn with_guest_stars(mut self, guest_stars: Vec<String>) -> Episode {
        self.guest_stars = Some(guest_stars);
        self
    }

    pub fn guest_stars(&self) -> Option<&Vec<String>> {
        self.guest_stars.as_ref()
    }

    pub fn reset_guest_stars(&mut self) {
        self.guest_stars = None;
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: i64) -> Episode {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&i64> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_imdb_id(&mut self, imdb_id: String) {
        self.imdb_id = Some(imdb_id);
    }

    pub fn with_imdb_id(mut self, imdb_id: String) -> Episode {
        self.imdb_id = Some(imdb_id);
        self
    }

    pub fn imdb_id(&self) -> Option<&String> {
        self.imdb_id.as_ref()
    }

    pub fn reset_imdb_id(&mut self) {
        self.imdb_id = None;
    }

    pub fn set_last_updated(&mut self, last_updated: i64) {
        self.last_updated = Some(last_updated);
    }

    pub fn with_last_updated(mut self, last_updated: i64) -> Episode {
        self.last_updated = Some(last_updated);
        self
    }

    pub fn last_updated(&self) -> Option<&i64> {
        self.last_updated.as_ref()
    }

    pub fn reset_last_updated(&mut self) {
        self.last_updated = None;
    }

    pub fn set_last_updated_by(&mut self, last_updated_by: i64) {
        self.last_updated_by = Some(last_updated_by);
    }

    pub fn with_last_updated_by(mut self, last_updated_by: i64) -> Episode {
        self.last_updated_by = Some(last_updated_by);
        self
    }

    pub fn last_updated_by(&self) -> Option<i64> {
        self.last_updated_by
    }

    pub fn reset_last_updated_by(&mut self) {
        self.last_updated_by = None;
    }

    pub fn set_overview(&mut self, overview: String) {
        self.overview = Some(overview);
    }

    pub fn with_overview(mut self, overview: String) -> Episode {
        self.overview = Some(overview);
        self
    }

    pub fn overview(&self) -> Option<&String> {
        self.overview.as_ref()
    }

    pub fn reset_overview(&mut self) {
        self.overview = None;
    }

    pub fn set_production_code(&mut self, production_code: String) {
        self.production_code = Some(production_code);
    }

    pub fn with_production_code(mut self, production_code: String) -> Episode {
        self.production_code = Some(production_code);
        self
    }

    pub fn production_code(&self) -> Option<&String> {
        self.production_code.as_ref()
    }

    pub fn reset_production_code(&mut self) {
        self.production_code = None;
    }

    pub fn set_series_id(&mut self, series_id: i64) {
        self.series_id = Some(series_id);
    }

    pub fn with_series_id(mut self, series_id: i64) -> Episode {
        self.series_id = Some(series_id);
        self
    }

    pub fn series_id(&self) -> Option<i64> {
        self.series_id
    }

    pub fn reset_series_id(&mut self) {
        self.series_id = None;
    }

    pub fn set_show_url(&mut self, show_url: String) {
        self.show_url = Some(show_url);
    }

    pub fn with_show_url(mut self, show_url: String) -> Episode {
        self.show_url = Some(show_url);
        self
    }

    pub fn show_url(&self) -> Option<&String> {
        self.show_url.as_ref()
    }

    pub fn reset_show_url(&mut self) {
        self.show_url = None;
    }

    pub fn set_site_rating(&mut self, site_rating: f64) {
        self.site_rating = Some(site_rating);
    }

    pub fn with_site_rating(mut self, site_rating: f64) -> Episode {
        self.site_rating = Some(site_rating);
        self
    }

    pub fn site_rating(&self) -> Option<&f64> {
        self.site_rating.as_ref()
    }

    pub fn reset_site_rating(&mut self) {
        self.site_rating = None;
    }

    pub fn set_site_rating_count(&mut self, site_rating_count: i64) {
        self.site_rating_count = Some(site_rating_count);
    }

    pub fn with_site_rating_count(mut self, site_rating_count: i64) -> Episode {
        self.site_rating_count = Some(site_rating_count);
        self
    }

    pub fn site_rating_count(&self) -> Option<&i64> {
        self.site_rating_count.as_ref()
    }

    pub fn reset_site_rating_count(&mut self) {
        self.site_rating_count = None;
    }

    pub fn set_thumb_added(&mut self, thumb_added: String) {
        self.thumb_added = Some(thumb_added);
    }

    pub fn with_thumb_added(mut self, thumb_added: String) -> Episode {
        self.thumb_added = Some(thumb_added);
        self
    }

    pub fn thumb_added(&self) -> Option<&String> {
        self.thumb_added.as_ref()
    }

    pub fn reset_thumb_added(&mut self) {
        self.thumb_added = None;
    }

    pub fn set_thumb_author(&mut self, thumb_author: i64) {
        self.thumb_author = Some(thumb_author);
    }

    pub fn with_thumb_author(mut self, thumb_author: i64) -> Episode {
        self.thumb_author = Some(thumb_author);
        self
    }

    pub fn thumb_author(&self) -> Option<&i64> {
        self.thumb_author.as_ref()
    }

    pub fn reset_thumb_author(&mut self) {
        self.thumb_author = None;
    }

    pub fn set_thumb_height(&mut self, thumb_height: String) {
        self.thumb_height = Some(thumb_height);
    }

    pub fn with_thumb_height(mut self, thumb_height: String) -> Episode {
        self.thumb_height = Some(thumb_height);
        self
    }

    pub fn thumb_height(&self) -> Option<&String> {
        self.thumb_height.as_ref()
    }

    pub fn reset_thumb_height(&mut self) {
        self.thumb_height = None;
    }

    pub fn set_thumb_width(&mut self, thumb_width: String) {
        self.thumb_width = Some(thumb_width);
    }

    pub fn with_thumb_width(mut self, thumb_width: String) -> Episode {
        self.thumb_width = Some(thumb_width);
        self
    }

    pub fn thumb_width(&self) -> Option<&String> {
        self.thumb_width.as_ref()
    }

    pub fn reset_thumb_width(&mut self) {
        self.thumb_width = None;
    }

    pub fn set_writers(&mut self, writers: Vec<String>) {
        self.writers = Some(writers);
    }

    pub fn with_writers(mut self, writers: Vec<String>) -> Episode {
        self.writers = Some(writers);
        self
    }

    pub fn writers(&self) -> Option<&Vec<String>> {
        self.writers.as_ref()
    }

    pub fn reset_writers(&mut self) {
        self.writers = None;
    }
}
