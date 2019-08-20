use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JsonErrors {
    /// Invalid filters passed to route
    #[serde(rename = "invalidFilters")]
    invalid_filters: Option<Vec<String>>,
    /// Invalid language or translation missing
    #[serde(rename = "invalidLanguage")]
    invalid_language: Option<String>,
    /// Invalid query params passed to route
    #[serde(rename = "invalidQueryParams")]
    invalid_query_params: Option<Vec<String>>,
}

impl JsonErrors {
    pub fn set_invalid_filters(&mut self, invalid_filters: Vec<String>) {
        self.invalid_filters = Some(invalid_filters);
    }

    pub fn with_invalid_filters(mut self, invalid_filters: Vec<String>) -> JsonErrors {
        self.invalid_filters = Some(invalid_filters);
        self
    }

    pub fn invalid_filters(&self) -> Option<&Vec<String>> {
        self.invalid_filters.as_ref()
    }

    pub fn reset_invalid_filters(&mut self) {
        self.invalid_filters = None;
    }

    pub fn set_invalid_language(&mut self, invalid_language: String) {
        self.invalid_language = Some(invalid_language);
    }

    pub fn with_invalid_language(mut self, invalid_language: String) -> JsonErrors {
        self.invalid_language = Some(invalid_language);
        self
    }

    pub fn invalid_language(&self) -> Option<&String> {
        self.invalid_language.as_ref()
    }

    pub fn reset_invalid_language(&mut self) {
        self.invalid_language = None;
    }

    pub fn set_invalid_query_params(&mut self, invalid_query_params: Vec<String>) {
        self.invalid_query_params = Some(invalid_query_params);
    }

    pub fn with_invalid_query_params(mut self, invalid_query_params: Vec<String>) -> JsonErrors {
        self.invalid_query_params = Some(invalid_query_params);
        self
    }

    pub fn invalid_query_params(&self) -> Option<&Vec<String>> {
        self.invalid_query_params.as_ref()
    }

    pub fn reset_invalid_query_params(&mut self) {
        self.invalid_query_params = None;
    }
}
