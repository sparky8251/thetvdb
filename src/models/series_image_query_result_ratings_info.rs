use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SeriesImageQueryResultRatingsInfo {
    /// Average rating for the given record.
    average: Option<f64>,
    /// Number of ratings for the given record.
    count: Option<u64>,
}

impl SeriesImageQueryResultRatingsInfo {
    pub fn set_average(&mut self, average: f64) {
        self.average = Some(average);
    }

    pub fn with_average(mut self, average: f64) -> SeriesImageQueryResultRatingsInfo {
        self.average = Some(average);
        self
    }

    pub fn average(&self) -> Option<&f64> {
        self.average.as_ref()
    }

    pub fn reset_average(&mut self) {
        self.average = None;
    }

    pub fn set_count(&mut self, count: u64) {
        self.count = Some(count);
    }

    pub fn with_count(mut self, count: u64) -> SeriesImageQueryResultRatingsInfo {
        self.count = Some(count);
        self
    }

    pub fn count(&self) -> Option<&u64> {
        self.count.as_ref()
    }

    pub fn reset_count(&mut self) {
        self.count = None;
    }
}
