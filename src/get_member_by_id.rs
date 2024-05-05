use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use crate::structs::get_member_by_id::Event;


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
