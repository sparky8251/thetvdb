use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SeriesImagesQueryParams {
    data: Option<Vec<crate::models::SeriesImagesQueryParam>>,
}

impl SeriesImagesQueryParams {
    pub fn set_data(&mut self, data: Vec<crate::models::SeriesImagesQueryParam>) {
        self.data = Some(data);
    }

    pub fn with_data(
        mut self,
        data: Vec<crate::models::SeriesImagesQueryParam>,
    ) -> SeriesImagesQueryParams {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&Vec<crate::models::SeriesImagesQueryParam>> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }
}
