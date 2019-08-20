use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Links {
    first: Option<i64>,
    last: Option<i64>,
    next: Option<i64>,
    previous: Option<i64>,
}

impl Links {
    pub fn set_first(&mut self, first: i64) {
        self.first = Some(first);
    }

    pub fn with_first(mut self, first: i64) -> Links {
        self.first = Some(first);
        self
    }

    pub fn first(&self) -> Option<&i64> {
        self.first.as_ref()
    }

    pub fn reset_first(&mut self) {
        self.first = None;
    }

    pub fn set_last(&mut self, last: i64) {
        self.last = Some(last);
    }

    pub fn with_last(mut self, last: i64) -> Links {
        self.last = Some(last);
        self
    }

    pub fn last(&self) -> Option<&i64> {
        self.last.as_ref()
    }

    pub fn reset_last(&mut self) {
        self.last = None;
    }

    pub fn set_next(&mut self, next: i64) {
        self.next = Some(next);
    }

    pub fn with_next(mut self, next: i64) -> Links {
        self.next = Some(next);
        self
    }

    pub fn next(&self) -> Option<&i64> {
        self.next.as_ref()
    }

    pub fn reset_next(&mut self) {
        self.next = None;
    }

    pub fn set_previous(&mut self, previous: i64) {
        self.previous = Some(previous);
    }

    pub fn with_previous(mut self, previous: i64) -> Links {
        self.previous = Some(previous);
        self
    }

    pub fn previous(&self) -> Option<&i64> {
        self.previous.as_ref()
    }

    pub fn reset_previous(&mut self) {
        self.previous = None;
    }
}
