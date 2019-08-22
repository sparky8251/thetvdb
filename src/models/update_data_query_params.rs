use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct UpdateDataQueryParams {
    from_time: i64,
    to_time: Option<i64>,
}

impl UpdateDataQueryParams {
    pub fn new(from_time: i64, to_time: Option<i64>) -> Self {
        // TODO: Implement bounds check on to_time such that it
        // is no more than 7 days after from_time as specified
        // in the API spec and then return a Result<Self, Error>
        // to make this apparent to API consumers
        UpdateDataQueryParams { from_time, to_time }
    }
}

impl Display for UpdateDataQueryParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.to_time {
            Some(v) => write!(
                f,
                "?fromTime={}&toTime={}",
                self.from_time.to_string(),
                v.to_string()
            ),
            None => write!(f, "?fromTime={}", self.from_time.to_string()),
        }
    }
}
