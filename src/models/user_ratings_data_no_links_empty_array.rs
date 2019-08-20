use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserRatingsDataNoLinksEmptyArray {
    // TODO: Verify correct data type
    data: Option<Vec<crate::models::JsonErrors>>,
}

impl UserRatingsDataNoLinksEmptyArray {
    // TODO: Verify correct data type
    pub fn set_data(&mut self, data: Vec<crate::models::JsonErrors>) {
        self.data = Some(data);
    }
    // TODO: Verify correct data type
    pub fn with_data(
        mut self,
        data: Vec<crate::models::JsonErrors>,
    ) -> UserRatingsDataNoLinksEmptyArray {
        self.data = Some(data);
        self
    }
    // TODO: Verify correct data type
    pub fn data(&self) -> Option<&Vec<crate::models::JsonErrors>> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }
}
