use std::thread::sleep;
use std::io::{Write, stdout};
use chrono::{DateTime, Duration, TimeDelta, Utc};

pub(crate) fn accurate_sleep(event_time: &str) -> TimeDelta {
    if let Ok(event_timestamp) = DateTime::parse_from_rfc3339(event_time) {
        loop
        {
            let current_time = Utc::now();
            let time_difference = event_timestamp.signed_duration_since(current_time);

            if time_difference.num_nanoseconds().unwrap() <= 0 {
                return time_difference;
            }

            if time_difference >= Duration::seconds(210) {
                print!("\rTime until api call: {} seconds       ", time_difference.num_seconds());
                stdout().flush().unwrap();
                sleep(std::time::Duration::from_secs(10));

            } else if time_difference >= Duration::seconds(3) {
                print!("\rTime until api call: {} seconds        ", time_difference.num_seconds());
                stdout().flush().unwrap();
                sleep(std::time::Duration::from_secs(1));

            } else if time_difference >= Duration::milliseconds(2) {
                print!("\rLaunch incoming! {:?} milliseconds     ", time_difference.num_milliseconds());
                stdout().flush().unwrap();
                sleep(std::time::Duration::from_millis(1))

            } else {
                // Busy waiting
            }
        }
    } else {
        panic!("Failed to parse event timestamp!")
    }
}
