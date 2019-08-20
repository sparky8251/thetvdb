use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Update {
    id: Option<i64>,
    #[serde(rename = "lastUpdated")]
    last_updated: Option<i64>,
}

impl Update {
    pub fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: i64) -> Update {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&i64> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_last_updated(&mut self, last_updated: i64) {
        self.last_updated = Some(last_updated);
    }

    pub fn with_last_updated(mut self, last_updated: i64) -> Update {
        self.last_updated = Some(last_updated);
        self
    }

    pub fn last_updated(&self) -> Option<&i64> {
        self.last_updated.as_ref()
    }

    pub fn reset_last_updated(&mut self) {
        self.last_updated = None;
    }
}
