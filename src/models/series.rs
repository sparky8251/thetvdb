use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Series {
    added: Option<String>,
    #[serde(rename = "airsDayOfWeek")]
    airs_day_of_week: Option<String>,
    #[serde(rename = "airsTime")]
    airs_time: Option<String>,
    aliases: Option<Vec<String>>,
    banner: Option<String>,
    #[serde(rename = "firstAired")]
    first_aired: Option<String>,
    genre: Option<Vec<String>>,
    id: Option<u64>,
    #[serde(rename = "imdbId")]
    imdb_id: Option<String>,
    #[serde(rename = "lastUpdated")]
    last_updated: Option<u64>,
    network: Option<String>,
    #[serde(rename = "networkId")]
    network_id: Option<String>,
    overview: Option<String>,
    rating: Option<String>,
    runtime: Option<String>,
    #[serde(rename = "seriesId")]
    series_id: Option<String>,
    #[serde(rename = "seriesName")]
    series_name: Option<String>,
    #[serde(rename = "siteRating")]
    site_rating: Option<f64>,
    #[serde(rename = "siteRatingCount")]
    site_rating_count: Option<u64>,
    slug: Option<String>,
    status: Option<String>,
    #[serde(rename = "zap2itId")]
    zap2it_id: Option<String>,
}

impl Series {
    pub fn set_added(&mut self, added: String) {
        self.added = Some(added);
    }

    pub fn with_added(mut self, added: String) -> Series {
        self.added = Some(added);
        self
    }

    pub fn added(&self) -> Option<&String> {
        self.added.as_ref()
    }

    pub fn reset_added(&mut self) {
        self.added = None;
    }

    pub fn set_airs_day_of_week(&mut self, airs_day_of_week: String) {
        self.airs_day_of_week = Some(airs_day_of_week);
    }

    pub fn with_airs_day_of_week(mut self, airs_day_of_week: String) -> Series {
        self.airs_day_of_week = Some(airs_day_of_week);
        self
    }

    pub fn airs_day_of_week(&self) -> Option<&String> {
        self.airs_day_of_week.as_ref()
    }

    pub fn reset_airs_day_of_week(&mut self) {
        self.airs_day_of_week = None;
    }

    pub fn set_airs_time(&mut self, airs_time: String) {
        self.airs_time = Some(airs_time);
    }

    pub fn with_airs_time(mut self, airs_time: String) -> Series {
        self.airs_time = Some(airs_time);
        self
    }

    pub fn airs_time(&self) -> Option<&String> {
        self.airs_time.as_ref()
    }

    pub fn reset_airs_time(&mut self) {
        self.airs_time = None;
    }

    pub fn set_aliases(&mut self, aliases: Vec<String>) {
        self.aliases = Some(aliases);
    }

    pub fn with_aliases(mut self, aliases: Vec<String>) -> Series {
        self.aliases = Some(aliases);
        self
    }

    pub fn aliases(&self) -> Option<&Vec<String>> {
        self.aliases.as_ref()
    }

    pub fn reset_aliases(&mut self) {
        self.aliases = None;
    }

    pub fn set_banner(&mut self, banner: String) {
        self.banner = Some(banner);
    }

    pub fn with_banner(mut self, banner: String) -> Series {
        self.banner = Some(banner);
        self
    }

    pub fn banner(&self) -> Option<&String> {
        self.banner.as_ref()
    }

    pub fn reset_banner(&mut self) {
        self.banner = None;
    }

    pub fn set_first_aired(&mut self, first_aired: String) {
        self.first_aired = Some(first_aired);
    }

    pub fn with_first_aired(mut self, first_aired: String) -> Series {
        self.first_aired = Some(first_aired);
        self
    }

    pub fn first_aired(&self) -> Option<&String> {
        self.first_aired.as_ref()
    }

    pub fn reset_first_aired(&mut self) {
        self.first_aired = None;
    }

    pub fn set_genre(&mut self, genre: Vec<String>) {
        self.genre = Some(genre);
    }

    pub fn with_genre(mut self, genre: Vec<String>) -> Series {
        self.genre = Some(genre);
        self
    }

    pub fn genre(&self) -> Option<&Vec<String>> {
        self.genre.as_ref()
    }

    pub fn reset_genre(&mut self) {
        self.genre = None;
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: u64) -> Series {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&u64> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_imdb_id(&mut self, imdb_id: String) {
        self.imdb_id = Some(imdb_id);
    }

    pub fn with_imdb_id(mut self, imdb_id: String) -> Series {
        self.imdb_id = Some(imdb_id);
        self
    }

    pub fn imdb_id(&self) -> Option<&String> {
        self.imdb_id.as_ref()
    }

    pub fn reset_imdb_id(&mut self) {
        self.imdb_id = None;
    }

    pub fn set_last_updated(&mut self, last_updated: u64) {
        self.last_updated = Some(last_updated);
    }

    pub fn with_last_updated(mut self, last_updated: u64) -> Series {
        self.last_updated = Some(last_updated);
        self
    }

    pub fn last_updated(&self) -> Option<&u64> {
        self.last_updated.as_ref()
    }

    pub fn reset_last_updated(&mut self) {
        self.last_updated = None;
    }

    pub fn set_network(&mut self, network: String) {
        self.network = Some(network);
    }

    pub fn with_network(mut self, network: String) -> Series {
        self.network = Some(network);
        self
    }

    pub fn network(&self) -> Option<&String> {
        self.network.as_ref()
    }

    pub fn reset_network(&mut self) {
        self.network = None;
    }

    pub fn set_network_id(&mut self, network_id: String) {
        self.network_id = Some(network_id);
    }

    pub fn with_network_id(mut self, network_id: String) -> Series {
        self.network_id = Some(network_id);
        self
    }

    pub fn network_id(&self) -> Option<&String> {
        self.network_id.as_ref()
    }

    pub fn reset_network_id(&mut self) {
        self.network_id = None;
    }

    pub fn set_overview(&mut self, overview: String) {
        self.overview = Some(overview);
    }

    pub fn with_overview(mut self, overview: String) -> Series {
        self.overview = Some(overview);
        self
    }

    pub fn overview(&self) -> Option<&String> {
        self.overview.as_ref()
    }

    pub fn reset_overview(&mut self) {
        self.overview = None;
    }

    pub fn set_rating(&mut self, rating: String) {
        self.rating = Some(rating);
    }

    pub fn with_rating(mut self, rating: String) -> Series {
        self.rating = Some(rating);
        self
    }

    pub fn rating(&self) -> Option<&String> {
        self.rating.as_ref()
    }

    pub fn reset_rating(&mut self) {
        self.rating = None;
    }

    pub fn set_runtime(&mut self, runtime: String) {
        self.runtime = Some(runtime);
    }

    pub fn with_runtime(mut self, runtime: String) -> Series {
        self.runtime = Some(runtime);
        self
    }

    pub fn runtime(&self) -> Option<&String> {
        self.runtime.as_ref()
    }

    pub fn reset_runtime(&mut self) {
        self.runtime = None;
    }

    pub fn set_series_id(&mut self, series_id: String) {
        self.series_id = Some(series_id);
    }

    pub fn with_series_id(mut self, series_id: String) -> Series {
        self.series_id = Some(series_id);
        self
    }

    pub fn series_id(&self) -> Option<&String> {
        self.series_id.as_ref()
    }

    pub fn reset_series_id(&mut self) {
        self.series_id = None;
    }

    pub fn set_series_name(&mut self, series_name: String) {
        self.series_name = Some(series_name);
    }

    pub fn with_series_name(mut self, series_name: String) -> Series {
        self.series_name = Some(series_name);
        self
    }

    pub fn series_name(&self) -> Option<&String> {
        self.series_name.as_ref()
    }

    pub fn reset_series_name(&mut self) {
        self.series_name = None;
    }

    pub fn set_site_rating(&mut self, site_rating: f64) {
        self.site_rating = Some(site_rating);
    }

    pub fn with_site_rating(mut self, site_rating: f64) -> Series {
        self.site_rating = Some(site_rating);
        self
    }

    pub fn site_rating(&self) -> Option<&f64> {
        self.site_rating.as_ref()
    }

    pub fn reset_site_rating(&mut self) {
        self.site_rating = None;
    }

    pub fn set_site_rating_count(&mut self, site_rating_count: u64) {
        self.site_rating_count = Some(site_rating_count);
    }

    pub fn with_site_rating_count(mut self, site_rating_count: u64) -> Series {
        self.site_rating_count = Some(site_rating_count);
        self
    }

    pub fn site_rating_count(&self) -> Option<&u64> {
        self.site_rating_count.as_ref()
    }

    pub fn reset_site_rating_count(&mut self) {
        self.site_rating_count = None;
    }

    pub fn set_slug(&mut self, slug: String) {
        self.slug = Some(slug);
    }

    pub fn with_slug(mut self, slug: String) -> Series {
        self.slug = Some(slug);
        self
    }

    pub fn slug(&self) -> Option<&String> {
        self.slug.as_ref()
    }

    pub fn reset_slug(&mut self) {
        self.slug = None;
    }

    pub fn set_status(&mut self, status: String) {
        self.status = Some(status);
    }

    pub fn with_status(mut self, status: String) -> Series {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&String> {
        self.status.as_ref()
    }

    pub fn reset_status(&mut self) {
        self.status = None;
    }

    pub fn set_zap2it_id(&mut self, zap2it_id: String) {
        self.zap2it_id = Some(zap2it_id);
    }

    pub fn with_zap2it_id(mut self, zap2it_id: String) -> Series {
        self.zap2it_id = Some(zap2it_id);
        self
    }

    pub fn zap2it_id(&self) -> Option<&String> {
        self.zap2it_id.as_ref()
    }

    pub fn reset_zap2it_id(&mut self) {
        self.zap2it_id = None;
    }
}
