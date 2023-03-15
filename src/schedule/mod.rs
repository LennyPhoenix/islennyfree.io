use std::{fs::File, io::Read};

use chrono::{DateTime, Duration, Utc};
use walkdir::WalkDir;

use self::activity::Activity;

pub mod activity;
pub mod timeframe;

pub fn load_schedule(path: &str) -> Vec<Activity> {
    let mut activities: Vec<Activity> = vec![];

    for entry in WalkDir::new(path).follow_links(true) {
        let entry = match entry {
            Ok(f) => f,
            Err(e) => {
                println!("Failed to walk `{path}`: {e}");
                continue;
            }
        };

        if entry.file_type().is_file() {
            let mut file = File::open(entry.path()).expect("open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("file to read");

            activities.push(serde_yaml::from_str(&contents).expect("file to deserealize"));
        }
    }

    activities
}

pub fn current_activity<'a>(
    now: &DateTime<Utc>,
    activities: &'a Vec<Activity>,
) -> Option<&'a Activity> {
    activities.iter().filter(|a| a.active(now)).next()
}

pub fn next_free(now: &DateTime<Utc>, activities: &Vec<Activity>) -> DateTime<Utc> {
    let mut test = now.clone();
    while let Some(current_activity) = current_activity(&test, activities) {
        let frame = current_activity.active_timeframe(&test).unwrap();
        test = test + frame.time_remaining(&test);
    }
    test
}
