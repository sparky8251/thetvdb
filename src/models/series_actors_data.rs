use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SeriesActorsData {
    id: Option<i64>,
    image: Option<String>,
    #[serde(rename = "imageAdded")]
    image_added: Option<String>,
    #[serde(rename = "imageAuthor")]
    image_author: Option<i64>,
    #[serde(rename = "lastUpdated")]
    last_updated: Option<String>,
    name: Option<String>,
    role: Option<String>,
    #[serde(rename = "seriesId")]
    series_id: Option<i64>,
    #[serde(rename = "sortOrder")]
    sort_order: Option<i64>,
}

impl SeriesActorsData {
    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: i64) -> SeriesActorsData {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&i64> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_image(&mut self, image: String) {
        self.image = Some(image);
    }

    pub fn with_image(mut self, image: String) -> SeriesActorsData {
        self.image = Some(image);
        self
    }

    pub fn image(&self) -> Option<&String> {
        self.image.as_ref()
    }

    pub fn reset_image(&mut self) {
        self.image = None;
    }

    pub fn set_image_added(&mut self, image_added: String) {
        self.image_added = Some(image_added);
    }

    pub fn with_image_added(mut self, image_added: String) -> SeriesActorsData {
        self.image_added = Some(image_added);
        self
    }

    pub fn image_added(&self) -> Option<&String> {
        self.image_added.as_ref()
    }

    pub fn reset_image_added(&mut self) {
        self.image_added = None;
    }

    pub fn set_image_author(&mut self, image_author: i64) {
        self.image_author = Some(image_author);
    }

    pub fn with_image_author(mut self, image_author: i64) -> SeriesActorsData {
        self.image_author = Some(image_author);
        self
    }

    pub fn image_author(&self) -> Option<&i64> {
        self.image_author.as_ref()
    }

    pub fn reset_image_author(&mut self) {
        self.image_author = None;
    }

    pub fn set_last_updated(&mut self, last_updated: String) {
        self.last_updated = Some(last_updated);
    }

    pub fn with_last_updated(mut self, last_updated: String) -> SeriesActorsData {
        self.last_updated = Some(last_updated);
        self
    }

    pub fn last_updated(&self) -> Option<&String> {
        self.last_updated.as_ref()
    }

    pub fn reset_last_updated(&mut self) {
        self.last_updated = None;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn with_name(mut self, name: String) -> SeriesActorsData {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_role(&mut self, role: String) {
        self.role = Some(role);
    }

    pub fn with_role(mut self, role: String) -> SeriesActorsData {
        self.role = Some(role);
        self
    }

    pub fn role(&self) -> Option<&String> {
        self.role.as_ref()
    }

    pub fn reset_role(&mut self) {
        self.role = None;
    }

    pub fn set_series_id(&mut self, series_id: i64) {
        self.series_id = Some(series_id);
    }

    pub fn with_series_id(mut self, series_id: i64) -> SeriesActorsData {
        self.series_id = Some(series_id);
        self
    }

    pub fn series_id(&self) -> Option<&i64> {
        self.series_id.as_ref()
    }

    pub fn reset_series_id(&mut self) {
        self.series_id = None;
    }

    pub fn set_sort_order(&mut self, sort_order: i64) {
        self.sort_order = Some(sort_order);
    }

    pub fn with_sort_order(mut self, sort_order: i64) -> SeriesActorsData {
        self.sort_order = Some(sort_order);
        self
    }

    pub fn sort_order(&self) -> Option<&i64> {
        self.sort_order.as_ref()
    }

    pub fn reset_sort_order(&mut self) {
        self.sort_order = None;
    }
}
