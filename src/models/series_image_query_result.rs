use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SeriesImageQueryResult {
    #[serde(rename = "fileName")]
    file_name: Option<String>,
    id: Option<i64>,
    #[serde(rename = "keyType")]
    key_type: Option<String>,
    #[serde(rename = "languageId")]
    language_id: Option<i64>,
    #[serde(rename = "ratingsInfo")]
    ratings_info: Option<crate::models::SeriesImageQueryResultRatingsInfo>,
    resolution: Option<String>,
    sub_key: Option<String>,
    thumbnail: Option<String>,
}

impl SeriesImageQueryResult {
    pub fn set_file_name(&mut self, file_name: String) {
        self.file_name = Some(file_name);
    }

    pub fn with_file_name(mut self, file_name: String) -> SeriesImageQueryResult {
        self.file_name = Some(file_name);
        self
    }

    pub fn file_name(&self) -> Option<&String> {
        self.file_name.as_ref()
    }

    pub fn reset_file_name(&mut self) {
        self.file_name = None;
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: i64) -> SeriesImageQueryResult {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&i64> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_key_type(&mut self, key_type: String) {
        self.key_type = Some(key_type);
    }

    pub fn with_key_type(mut self, key_type: String) -> SeriesImageQueryResult {
        self.key_type = Some(key_type);
        self
    }

    pub fn key_type(&self) -> Option<&String> {
        self.key_type.as_ref()
    }

    pub fn reset_key_type(&mut self) {
        self.key_type = None;
    }

    pub fn set_language_id(&mut self, language_id: i64) {
        self.language_id = Some(language_id);
    }

    pub fn with_language_id(mut self, language_id: i64) -> SeriesImageQueryResult {
        self.language_id = Some(language_id);
        self
    }

    pub fn language_id(&self) -> Option<&i64> {
        self.language_id.as_ref()
    }

    pub fn reset_language_id(&mut self) {
        self.language_id = None;
    }

    pub fn set_ratings_info(
        &mut self,
        ratings_info: crate::models::SeriesImageQueryResultRatingsInfo,
    ) {
        self.ratings_info = Some(ratings_info);
    }

    pub fn with_ratings_info(
        mut self,
        ratings_info: crate::models::SeriesImageQueryResultRatingsInfo,
    ) -> SeriesImageQueryResult {
        self.ratings_info = Some(ratings_info);
        self
    }

    pub fn ratings_info(&self) -> Option<&crate::models::SeriesImageQueryResultRatingsInfo> {
        self.ratings_info.as_ref()
    }

    pub fn reset_ratings_info(&mut self) {
        self.ratings_info = None;
    }

    pub fn set_resolution(&mut self, resolution: String) {
        self.resolution = Some(resolution);
    }

    pub fn with_resolution(mut self, resolution: String) -> SeriesImageQueryResult {
        self.resolution = Some(resolution);
        self
    }

    pub fn resolution(&self) -> Option<&String> {
        self.resolution.as_ref()
    }

    pub fn reset_resolution(&mut self) {
        self.resolution = None;
    }

    pub fn set_sub_key(&mut self, sub_key: String) {
        self.sub_key = Some(sub_key);
    }

    pub fn with_sub_key(mut self, sub_key: String) -> SeriesImageQueryResult {
        self.sub_key = Some(sub_key);
        self
    }

    pub fn sub_key(&self) -> Option<&String> {
        self.sub_key.as_ref()
    }

    pub fn reset_sub_key(&mut self) {
        self.sub_key = None;
    }

    pub fn set_thumbnail(&mut self, thumbnail: String) {
        self.thumbnail = Some(thumbnail);
    }

    pub fn with_thumbnail(mut self, thumbnail: String) -> SeriesImageQueryResult {
        self.thumbnail = Some(thumbnail);
        self
    }

    pub fn thumbnail(&self) -> Option<&String> {
        self.thumbnail.as_ref()
    }

    pub fn reset_thumbnail(&mut self) {
        self.thumbnail = None;
    }
}
