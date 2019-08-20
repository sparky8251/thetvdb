use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "favoritesDisplaymode")]
    favorites_displaymode: Option<String>,
    language: Option<String>,
    #[serde(rename = "userName")]
    user_name: Option<String>,
}

impl User {
    pub fn set_favorites_displaymode(&mut self, favorites_displaymode: String) {
        self.favorites_displaymode = Some(favorites_displaymode);
    }

    pub fn with_favorites_displaymode(mut self, favorites_displaymode: String) -> User {
        self.favorites_displaymode = Some(favorites_displaymode);
        self
    }

    pub fn favorites_displaymode(&self) -> Option<&String> {
        self.favorites_displaymode.as_ref()
    }

    pub fn reset_favorites_displaymode(&mut self) {
        self.favorites_displaymode = None;
    }

    pub fn set_language(&mut self, language: String) {
        self.language = Some(language);
    }

    pub fn with_language(mut self, language: String) -> User {
        self.language = Some(language);
        self
    }

    pub fn language(&self) -> Option<&String> {
        self.language.as_ref()
    }

    pub fn reset_language(&mut self) {
        self.language = None;
    }

    pub fn set_user_name(&mut self, user_name: String) {
        self.user_name = Some(user_name);
    }

    pub fn with_user_name(mut self, user_name: String) -> User {
        self.user_name = Some(user_name);
        self
    }

    pub fn user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }

    pub fn reset_user_name(&mut self) {
        self.user_name = None;
    }
}
