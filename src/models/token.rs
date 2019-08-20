use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Token {
    token: Option<String>,
}

impl Token {
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub fn with_token(mut self, token: String) -> Token {
        self.token = Some(token);
        self
    }

    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }

    pub fn reset_token(&mut self) {
        self.token = None;
    }
}
