use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SeriesActors {
    data: Option<Vec<crate::models::SeriesActorsData>>,
    errors: Option<crate::models::JsonErrors>,
}

impl SeriesActors {
    pub fn set_data(&mut self, data: Vec<crate::models::SeriesActorsData>) {
        self.data = Some(data);
    }

    pub fn with_data(mut self, data: Vec<crate::models::SeriesActorsData>) -> SeriesActors {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&Vec<crate::models::SeriesActorsData>> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }

    pub fn set_errors(&mut self, errors: crate::models::JsonErrors) {
        self.errors = Some(errors);
    }

    pub fn with_errors(mut self, errors: crate::models::JsonErrors) -> SeriesActors {
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
