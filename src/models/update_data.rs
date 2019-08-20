use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateData {
    data: Option<Vec<crate::models::Update>>,
    errors: Option<crate::models::JsonErrors>,
}

impl UpdateData {
    pub fn set_data(&mut self, data: Vec<crate::models::Update>) {
        self.data = Some(data);
    }

    pub fn with_data(mut self, data: Vec<crate::models::Update>) -> UpdateData {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&Vec<crate::models::Update>> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }

    pub fn set_errors(&mut self, errors: crate::models::JsonErrors) {
        self.errors = Some(errors);
    }

    pub fn with_errors(mut self, errors: crate::models::JsonErrors) -> UpdateData {
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
