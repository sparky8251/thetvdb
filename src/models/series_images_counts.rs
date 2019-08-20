use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SeriesImagesCounts {
    data: Option<crate::models::SeriesImagesCount>,
}

impl SeriesImagesCounts {
    pub fn set_data(&mut self, data: crate::models::SeriesImagesCount) {
        self.data = Some(data);
    }

    pub fn with_data(mut self, data: crate::models::SeriesImagesCount) -> SeriesImagesCounts {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&crate::models::SeriesImagesCount> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }
}
