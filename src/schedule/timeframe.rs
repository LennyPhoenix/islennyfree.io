use chrono::{prelude::*, Duration};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeFrame {
    day: Weekday,
    start: NaiveTime,
    end: NaiveTime,
}

impl TimeFrame {
    pub fn from_start_end(day: Weekday, start: NaiveTime, end: NaiveTime) -> Self {
        Self { day, start, end }
    }

    pub fn from_start_duration(day: Weekday, start: NaiveTime, duration: Duration) -> Self {
        Self {
            day,
            start,
            end: start + duration,
        }
    }

    pub fn today(&self, now: &DateTime<Utc>) -> bool {
        self.day == now.weekday()
    }

    pub fn duration(&self) -> Duration {
        self.end - self.start
    }

    pub fn start(&self) -> &NaiveTime {
        &self.start
    }

    pub fn end(&self) -> &NaiveTime {
        &self.end
    }

    pub fn time_remaining(&self, now: &DateTime<Utc>) -> Duration {
        Duration::seconds(
            (self.end.num_seconds_from_midnight() - now.num_seconds_from_midnight()).into(),
        )
    }

    pub fn active(&self, now: &DateTime<Utc>) -> bool {
        if self.today(now) {
            let ts = now.num_seconds_from_midnight();
            let start = self.start.num_seconds_from_midnight();
            let end = self.end.num_seconds_from_midnight();

            start <= ts && ts < end
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TimeFrame;
    use chrono::{prelude::*, Duration};

    #[test]
    fn active() {
        // Wednesday, 15th March 2023
        // 11:16:51 (AM)
        let test_date = Utc.with_ymd_and_hms(2023, 3, 15, 11, 16, 51).unwrap();

        let frame = TimeFrame {
            day: Weekday::Wed,
            start: NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
            end: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        };

        assert!(frame.active(&test_date));
    }

    #[test]
    fn inactive() {
        // Thursday, 16th March 2023
        // 9:11:12 (AM)
        let test_date = Utc.with_ymd_and_hms(2023, 3, 16, 9, 11, 12).unwrap();

        let frame = TimeFrame {
            day: Weekday::Wed,
            start: NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
            end: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        };

        assert!(!frame.active(&test_date));
    }

    #[test]
    fn time_remaining() {
        // Wednesday, 15th March 2023
        // 11:16:51 (AM)
        let test_date = Utc.with_ymd_and_hms(2023, 3, 15, 11, 16, 51).unwrap();

        let frame = TimeFrame {
            day: Weekday::Wed,
            start: NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
            end: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        };

        assert_eq!(
            frame.time_remaining(&test_date),
            Duration::seconds(9 + 60 * 43)
        );
    }

    #[test]
    fn duration() {
        let frame = TimeFrame {
            day: Weekday::Wed,
            start: NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
            end: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        };
        assert_eq!(frame.duration(), Duration::hours(1));

        let frame = TimeFrame {
            day: Weekday::Wed,
            start: NaiveTime::from_hms_opt(9, 45, 0).unwrap(),
            end: NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
        };
        assert_eq!(frame.duration(), Duration::minutes(75));
    }
}
