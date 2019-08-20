use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserData {
    data: Option<crate::models::User>,
}

impl UserData {
    pub fn set_data(&mut self, data: crate::models::User) {
        self.data = Some(data);
    }

    pub fn with_data(mut self, data: crate::models::User) -> UserData {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&crate::models::User> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }
}
