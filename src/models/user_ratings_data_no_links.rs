use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserRatingsDataNoLinks {
    data: Option<Vec<crate::models::UserRatings>>,
}

impl UserRatingsDataNoLinks {
    pub fn set_data(&mut self, data: Vec<crate::models::UserRatings>) {
        self.data = Some(data);
    }

    pub fn with_data(mut self, data: Vec<crate::models::UserRatings>) -> UserRatingsDataNoLinks {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&Vec<crate::models::UserRatings>> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }
}
