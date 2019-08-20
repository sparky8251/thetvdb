use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SeriesSearchResult {
    aliases: Option<Vec<String>>,
    banner: Option<String>,
    #[serde(rename = "firstAired")]
    first_aired: Option<String>,
    id: Option<i64>,
    network: Option<String>,
    overview: Option<String>,
    #[serde(rename = "seriesName")]
    series_name: Option<String>,
    slug: Option<String>,
    status: Option<String>,
}

impl SeriesSearchResult {
    pub fn set_aliases(&mut self, aliases: Vec<String>) {
        self.aliases = Some(aliases);
    }

    pub fn with_aliases(mut self, aliases: Vec<String>) -> SeriesSearchResult {
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

    pub fn with_banner(mut self, banner: String) -> SeriesSearchResult {
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

    pub fn with_first_aired(mut self, first_aired: String) -> SeriesSearchResult {
        self.first_aired = Some(first_aired);
        self
    }

    pub fn first_aired(&self) -> Option<&String> {
        self.first_aired.as_ref()
    }

    pub fn reset_first_aired(&mut self) {
        self.first_aired = None;
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: i64) -> SeriesSearchResult {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&i64> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_network(&mut self, network: String) {
        self.network = Some(network);
    }

    pub fn with_network(mut self, network: String) -> SeriesSearchResult {
        self.network = Some(network);
        self
    }

    pub fn network(&self) -> Option<&String> {
        self.network.as_ref()
    }

    pub fn reset_network(&mut self) {
        self.network = None;
    }

    pub fn set_overview(&mut self, overview: String) {
        self.overview = Some(overview);
    }

    pub fn with_overview(mut self, overview: String) -> SeriesSearchResult {
        self.overview = Some(overview);
        self
    }

    pub fn overview(&self) -> Option<&String> {
        self.overview.as_ref()
    }

    pub fn reset_overview(&mut self) {
        self.overview = None;
    }

    pub fn set_series_name(&mut self, series_name: String) {
        self.series_name = Some(series_name);
    }

    pub fn with_series_name(mut self, series_name: String) -> SeriesSearchResult {
        self.series_name = Some(series_name);
        self
    }

    pub fn series_name(&self) -> Option<&String> {
        self.series_name.as_ref()
    }

    pub fn reset_series_name(&mut self) {
        self.series_name = None;
    }

    pub fn set_slug(&mut self, slug: String) {
        self.slug = Some(slug);
    }

    pub fn with_slug(mut self, slug: String) -> SeriesSearchResult {
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

    pub fn with_status(mut self, status: String) -> SeriesSearchResult {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&String> {
        self.status.as_ref()
    }

    pub fn reset_status(&mut self) {
        self.status = None;
    }
}
