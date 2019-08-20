use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserRatingsData {
    data: Option<Vec<crate::models::UserRatings>>,
    errors: Option<crate::models::JsonErrors>,
    links: Option<crate::models::Links>,
}

impl UserRatingsData {
    pub fn set_data(&mut self, data: Vec<crate::models::UserRatings>) {
        self.data = Some(data);
    }

    pub fn with_data(mut self, data: Vec<crate::models::UserRatings>) -> UserRatingsData {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&Vec<crate::models::UserRatings>> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }

    pub fn set_errors(&mut self, errors: crate::models::JsonErrors) {
        self.errors = Some(errors);
    }

    pub fn with_errors(mut self, errors: crate::models::JsonErrors) -> UserRatingsData {
        self.errors = Some(errors);
        self
    }

    pub fn errors(&self) -> Option<&crate::models::JsonErrors> {
        self.errors.as_ref()
    }

    pub fn reset_errors(&mut self) {
        self.errors = None;
    }

    pub fn set_links(&mut self, links: crate::models::Links) {
        self.links = Some(links);
    }

    pub fn with_links(mut self, links: crate::models::Links) -> UserRatingsData {
        self.links = Some(links);
        self
    }

    pub fn links(&self) -> Option<&crate::models::Links> {
        self.links.as_ref()
    }

    pub fn reset_links(&mut self) {
        self.links = None;
    }
}
