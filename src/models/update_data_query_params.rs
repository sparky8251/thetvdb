use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateDataQueryParams {
    data: Option<Vec<String>>,
}

impl UpdateDataQueryParams {
    pub fn set_data(&mut self, data: Vec<String>) {
        self.data = Some(data);
    }

    pub fn with_data(mut self, data: Vec<String>) -> UpdateDataQueryParams {
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
