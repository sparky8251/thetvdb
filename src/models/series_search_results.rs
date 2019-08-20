use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SeriesSearchResults {
    data: Option<Vec<crate::models::SeriesSearchResult>>,
}

impl SeriesSearchResults {
    pub fn set_data(&mut self, data: Vec<crate::models::SeriesSearchResult>) {
        self.data = Some(data);
    }

    pub fn with_data(
        mut self,
        data: Vec<crate::models::SeriesSearchResult>,
    ) -> SeriesSearchResults {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&Vec<crate::models::SeriesSearchResult>> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }
}
