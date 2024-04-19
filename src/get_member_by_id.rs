use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Member {
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Group {
    members: Vec<Member>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Recipient {
    group: Group,
}

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    recipients: Recipient,
    // Include other fields from your JSON structure as needed
}

fn read_json<P: AsRef<Path>>(path: P) -> serde_json::Result<Vec<Event>> {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read the file");

    serde_json::from_str(&contents)
}

pub(crate) fn get_name_by_id(file_path: &str, id: &str) -> Option<(String, String)> {
    let events: Vec<Event> = read_json(file_path).expect("Failed to parse JSON");

    for event in events {
        for member in event.recipients.group.members {
            if member.id == id {
                return Some((member.first_name, member.last_name));
            }
        }
    }

    None
}
