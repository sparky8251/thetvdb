use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NotFound {
    #[serde(rename = "Error")]
    error: Option<String>,
}

impl NotFound {
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    pub fn with_error(mut self, error: String) -> NotFound {
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
