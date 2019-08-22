use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserRatings {
    rating: Option<u64>,
    #[serde(rename = "ratingItemId")]
    rating_item_id: Option<u64>,
    #[serde(rename = "ratingType")]
    rating_type: Option<String>,
}

impl UserRatings {
    pub fn set_rating(&mut self, rating: u64) {
        self.rating = Some(rating);
    }

    pub fn with_rating(mut self, rating: u64) -> UserRatings {
        self.rating = Some(rating);
        self
    }

    pub fn rating(&self) -> Option<&u64> {
        self.rating.as_ref()
    }

    pub fn reset_rating(&mut self) {
        self.rating = None;
    }

    pub fn set_rating_item_id(&mut self, rating_item_id: u64) {
        self.rating_item_id = Some(rating_item_id);
    }

    pub fn with_rating_item_id(mut self, rating_item_id: u64) -> UserRatings {
        self.rating_item_id = Some(rating_item_id);
        self
    }

    pub fn rating_item_id(&self) -> Option<&u64> {
        self.rating_item_id.as_ref()
    }

    pub fn reset_rating_item_id(&mut self) {
        self.rating_item_id = None;
    }

    pub fn set_rating_type(&mut self, rating_type: String) {
        self.rating_type = Some(rating_type);
    }

    pub fn with_rating_type(mut self, rating_type: String) -> UserRatings {
        self.rating_type = Some(rating_type);
        self
    }

    pub fn rating_type(&self) -> Option<&String> {
        self.rating_type.as_ref()
    }

    pub fn reset_rating_type(&mut self) {
        self.rating_type = None;
    }
}
