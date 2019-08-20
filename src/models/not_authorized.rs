use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NotAuthorized {
    #[serde(rename = "Error")]
    error: Option<String>,
}

impl NotAuthorized {
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    pub fn with_error(mut self, error: String) -> NotAuthorized {
        self.error = Some(error);
        self
    }

    pub fn error(&self) -> Option<&String> {
        self.error.as_ref()
    }

    pub fn reset_error(&mut self) {
        self.error = None;
    }
}
