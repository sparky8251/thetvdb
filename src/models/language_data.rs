use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LanguageData {
    data: Option<Vec<crate::models::Language>>,
}

impl LanguageData {
    pub fn set_data(&mut self, data: Vec<crate::models::Language>) {
        self.data = Some(data);
    }

    pub fn with_data(mut self, data: Vec<crate::models::Language>) -> LanguageData {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&Vec<crate::models::Language>> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }
}
