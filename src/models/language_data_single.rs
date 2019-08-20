use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LanguageDataSingle {
    data: Option<crate::models::Language>,
}

impl LanguageDataSingle {
    pub fn set_data(&mut self, data: crate::models::Language) {
        self.data = Some(data);
    }

    pub fn with_data(mut self, data: crate::models::Language) -> LanguageDataSingle {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&crate::models::Language> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }
}
