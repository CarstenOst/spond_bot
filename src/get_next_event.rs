use serde_json::Value;
use std::{error::Error, fs};

pub(crate) fn get_next_event(filename: &str) -> Result<Option<(String, Option<String>, String)>, Box<dyn Error>> {
    // Read the JSON file
    let file_content = fs::read_to_string(filename)?;

    // Parse the JSON string
    let json_data: Value = serde_json::from_str(&file_content)?;

    // Create a reference to the JSON array
    let events: Vec<&Value> = if let Value::Array(ref arr) = json_data {
        arr.iter().collect()
    } else {
        return Err("Invalid JSON format".into());
    };

    // Find the closest event in the future
    let mut next_event: Option<(String, Option<String>, String)> = None;
    let current_timestamp = chrono::Utc::now().to_rfc3339(); // Get current timestamp
    let mut min_diff: Option<i64> = None;
    for event in events {
        if let Some(heading) = event.get("heading").and_then(|v| v.as_str()) {
            if heading.contains("kurs") {
                println!("I'm skipping {:?}", event);
                continue;
            }
        }

        let invite_time = event.get("inviteTime").and_then(|v| v.as_str());
        //println!("Next invite time is {:?}", invite_time);
        if let Some(invite_time) = invite_time {
            let event_timestamp = chrono::DateTime::parse_from_rfc3339(invite_time)?;
            let diff = event_timestamp.signed_duration_since(chrono::DateTime::parse_from_rfc3339(&current_timestamp)?).num_seconds().abs();
            if min_diff.map_or(true, |d| diff < d) {
                min_diff = Some(diff);
                next_event = Some((
                    event.get("id").map_or("none", |v| v.as_str().unwrap_or("none")).to_string(),
                    Some(invite_time.to_string()),
                    event.get("heading").map_or("none", |v| v.as_str().unwrap_or("none")).to_string()
                ));

            }
        }
    }

    if let Some((id, invite_time, header)) = &next_event {
        println!("Next event '{}' ID found: {}, with invite Time: {}", &header.as_str(), &id, &invite_time.as_deref().unwrap_or("none"));
    } else {
        println!("No next event found, consider shutting off the script and take a summer vacation");
        panic!("Shutting down ^^^^ ")
        // TODO make it sleep for a week at a time
    }

    Ok(next_event)
}
