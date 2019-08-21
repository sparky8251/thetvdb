use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum SeriesSearchQueryParams {
    Name(String),
    ImdbId(String),
    Zap2ItId(String),
    Slug(String),
}

impl Display for SeriesSearchQueryParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Name(n) => write!(f, "?name={}", n),
            Self::ImdbId(i) => write!(f, "?imdbId={}", i),
            Self::Zap2ItId(z) => write!(f, "?zap2itId={}", z),
            Self::Slug(s) => write!(f, "?slug={}", s),
        }
    }
}
