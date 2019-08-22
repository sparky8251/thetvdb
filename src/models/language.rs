use serde::{self, Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Language {
    abbreviation: Option<String>,
    #[serde(rename = "englishName")]
    english_name: Option<String>,
    id: Option<u64>,
    name: Option<String>,
}

impl Language {
    pub fn set_abbreviation(&mut self, abbreviation: String) {
        self.abbreviation = Some(abbreviation);
    }

    pub fn with_abbreviation(mut self, abbreviation: String) -> Language {
        self.abbreviation = Some(abbreviation);
        self
    }

    pub fn abbreviation(&self) -> Option<&String> {
        self.abbreviation.as_ref()
    }

    pub fn reset_abbreviation(&mut self) {
        self.abbreviation = None;
    }

    pub fn set_english_name(&mut self, english_name: String) {
        self.english_name = Some(english_name);
    }

    pub fn with_english_name(mut self, english_name: String) -> Language {
        self.english_name = Some(english_name);
        self
    }

    pub fn english_name(&self) -> Option<&String> {
        self.english_name.as_ref()
    }

    pub fn reset_english_name(&mut self) {
        self.english_name = None;
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: u64) -> Language {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&u64> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn with_name(mut self, name: String) -> Language {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }
}
