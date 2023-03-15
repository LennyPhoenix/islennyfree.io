use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::timeframe::TimeFrame;

#[derive(Deserialize, Serialize, Debug)]
pub struct Activity {
    name: String,
    times: Vec<TimeFrame>,
}

impl Activity {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn times(&self) -> &Vec<TimeFrame> {
        &self.times
    }

    pub fn active_timeframe(&self, now: &DateTime<Utc>) -> Option<&TimeFrame> {
        self.times.iter().filter(|frame| frame.active(&now)).next()
    }

    pub fn active(&self, now: &DateTime<Utc>) -> bool {
        self.active_timeframe(now).is_some()
    }
}

// Roasted Potato
