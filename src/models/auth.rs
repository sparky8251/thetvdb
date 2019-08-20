use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Auth {
    apikey: Option<String>,
    userkey: Option<String>,
    username: Option<String>,
}

impl Auth {
    pub fn set_apikey(&mut self, apikey: String) {
        self.apikey = Some(apikey);
    }

    pub fn with_apikey(mut self, apikey: String) -> Auth {
        self.apikey = Some(apikey);
        self
    }

    pub fn apikey(&self) -> Option<&String> {
        self.apikey.as_ref()
    }

    pub fn reset_apikey(&mut self) {
        self.apikey = None;
    }

    pub fn set_userkey(&mut self, userkey: String) {
        self.userkey = Some(userkey);
    }

    pub fn with_userkey(mut self, userkey: String) -> Auth {
        self.userkey = Some(userkey);
        self
    }

    pub fn userkey(&self) -> Option<&String> {
        self.userkey.as_ref()
    }

    pub fn reset_userkey(&mut self) {
        self.userkey = None;
    }

    pub fn set_username(&mut self, username: String) {
        self.username = Some(username);
    }

    pub fn with_username(mut self, username: String) -> Auth {
        self.username = Some(username);
        self
    }

    pub fn username(&self) -> Option<&String> {
        self.username.as_ref()
    }

    pub fn reset_username(&mut self) {
        self.username = None;
    }
}
