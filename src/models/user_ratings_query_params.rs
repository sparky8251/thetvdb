use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserRatingsQueryParams {
    data: Option<Vec<String>>,
}

impl UserRatingsQueryParams {
    pub fn set_data(&mut self, data: Vec<String>) {
        self.data = Some(data);
    }

    pub fn with_data(mut self, data: Vec<String>) -> UserRatingsQueryParams {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&Vec<String>> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }
}
