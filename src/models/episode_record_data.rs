use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EpisodeRecordData {
    data: Option<crate::models::Episode>,
    errors: Option<crate::models::JsonErrors>,
}

impl EpisodeRecordData {
    pub fn set_data(&mut self, data: crate::models::Episode) {
        self.data = Some(data);
    }

    pub fn with_data(mut self, data: crate::models::Episode) -> EpisodeRecordData {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&crate::models::Episode> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }

    pub fn set_errors(&mut self, errors: crate::models::JsonErrors) {
        self.errors = Some(errors);
    }

    pub fn with_errors(mut self, errors: crate::models::JsonErrors) -> EpisodeRecordData {
        self.errors = Some(errors);
        self
    }

    pub fn errors(&self) -> Option<&crate::models::JsonErrors> {
        self.errors.as_ref()
    }

    pub fn reset_errors(&mut self) {
        self.errors = None;
    }
}
