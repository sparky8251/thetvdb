use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SeriesImagesQueryParam {
    #[serde(rename = "keyType")]
    key_type: Option<String>,
    #[serde(rename = "languageId")]
    language_id: Option<String>,
    resolution: Option<Vec<String>>,
    sub_key: Option<Vec<String>>,
}

impl SeriesImagesQueryParam {
    pub fn set_key_type(&mut self, key_type: String) {
        self.key_type = Some(key_type);
    }

    pub fn with_key_type(mut self, key_type: String) -> SeriesImagesQueryParam {
        self.key_type = Some(key_type);
        self
    }

    pub fn key_type(&self) -> Option<&String> {
        self.key_type.as_ref()
    }

    pub fn reset_key_type(&mut self) {
        self.key_type = None;
    }

    pub fn set_language_id(&mut self, language_id: String) {
        self.language_id = Some(language_id);
    }

    pub fn with_language_id(mut self, language_id: String) -> SeriesImagesQueryParam {
        self.language_id = Some(language_id);
        self
    }

    pub fn language_id(&self) -> Option<&String> {
        self.language_id.as_ref()
    }

    pub fn reset_language_id(&mut self) {
        self.language_id = None;
    }

    pub fn set_resolution(&mut self, resolution: Vec<String>) {
        self.resolution = Some(resolution);
    }

    pub fn with_resolution(mut self, resolution: Vec<String>) -> SeriesImagesQueryParam {
        self.resolution = Some(resolution);
        self
    }

    pub fn resolution(&self) -> Option<&Vec<String>> {
        self.resolution.as_ref()
    }

    pub fn reset_resolution(&mut self) {
        self.resolution = None;
    }

    pub fn set_sub_key(&mut self, sub_key: Vec<String>) {
        self.sub_key = Some(sub_key);
    }

    pub fn with_sub_key(mut self, sub_key: Vec<String>) -> SeriesImagesQueryParam {
        self.sub_key = Some(sub_key);
        self
    }

    pub fn sub_key(&self) -> Option<&Vec<String>> {
        self.sub_key.as_ref()
    }

    pub fn reset_sub_key(&mut self) {
        self.sub_key = None;
    }
}
