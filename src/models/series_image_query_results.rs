use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SeriesImageQueryResults {
    data: Option<Vec<crate::models::SeriesImageQueryResult>>,
    errors: Option<crate::models::JsonErrors>,
}

impl SeriesImageQueryResults {
    pub fn set_data(&mut self, data: Vec<crate::models::SeriesImageQueryResult>) {
        self.data = Some(data);
    }

    pub fn with_data(
        mut self,
        data: Vec<crate::models::SeriesImageQueryResult>,
    ) -> SeriesImageQueryResults {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&Vec<crate::models::SeriesImageQueryResult>> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }

    pub fn set_errors(&mut self, errors: crate::models::JsonErrors) {
        self.errors = Some(errors);
    }

    pub fn with_errors(mut self, errors: crate::models::JsonErrors) -> SeriesImageQueryResults {
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
