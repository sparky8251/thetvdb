use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserFavorites {
    favorites: Option<Vec<String>>,
}

impl UserFavorites {
    pub fn set_favorites(&mut self, favorites: Vec<String>) {
        self.favorites = Some(favorites);
    }

    pub fn with_favorites(mut self, favorites: Vec<String>) -> UserFavorites {
        self.favorites = Some(favorites);
        self
    }

    pub fn favorites(&self) -> Option<&Vec<String>> {
        self.favorites.as_ref()
    }

    pub fn reset_favorites(&mut self) {
        self.favorites = None;
    }
}
