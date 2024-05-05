use reqwest::blocking::Client;
use std::{error::Error, fs, process};
use serde_json::{json, Value};

pub(crate) fn update_events(url: &str, bearer_token: &str) -> Result<(), Box<dyn Error>> {
    // Create a reqwest client
    let client = Client::new();

    // Make a GET request to the API endpoint with the bearer token
    let response = client.get(url)
        .bearer_auth(bearer_token)
        .send()?;

    // Ensure the response is successful (status code 200)
    if response.status().is_success() {
        // Read the response body as text
        let response_text = response.text()?;

        // Parse the JSON response string
        let json_value: Value = serde_json::from_str(&response_text)?;

        // Serialize the JSON value with pretty printing
        let pretty_json = serde_json::to_string_pretty(&json_value)?;

        // Write the pretty-printed JSON to a file
        fs::write("response.json", pretty_json)?;

        read_ids_and_invite_times("response.json").expect("Where not able to update events.json (27)");

        //println!("API response successfully stored.");
    } else {
        // If the response is not successful, print the status code
        println!("Error: API request failed with status code {}, ", response.status());
        println!("Check that your bearer token '{}' is correct", bearer_token);
        println!("Check that ur group ID is correct in this url: {}", url);
        println!("Could not update events -> shutting down");
        process::exit(1);
    }

    Ok(())
}


fn read_ids_and_invite_times(filename: &str) -> Result<(), Box<dyn Error>> {
    // Read the JSON file
    let file_content = fs::read_to_string(filename)?;
    // Parse the JSON string
    let json_data: Value = serde_json::from_str(&file_content)?;

    // Extract "id" and "inviteTime" from each object in the array
    let result = if let Value::Array(arr) = json_data {
        arr.into_iter()
            .filter_map(|val| {
                if let Value::Object(obj) = val {
                    let id = obj.get("id")?.as_str()?.to_string();
                    let invite_time = obj.get("inviteTime").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let heading = obj.get("heading")?.as_str()?.to_string();
                    Some((id, invite_time, heading))
                } else {
                    None
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    Ok(write_events_to_file(result)?)
}
fn write_events_to_file(events: Vec<(String, Option<String>, String)>) -> Result<(), Box<dyn Error>> {
    // Create a JSON array to store the extracted data
    let mut events_json = Vec::new();
    for (id, invite_time, heading) in events {
        // Create a JSON object for each event
        let event_json = json!({
            "id": id,
            "inviteTime": invite_time,
            "heading": heading
        });
        events_json.push(event_json);
    }

    // Write the JSON array to a new file called "events.json"
    let events_file_content = serde_json::to_string_pretty(&events_json)?;
    fs::write("events.json", events_file_content)?;

    //println!("Events data successfully stored in events.json");

    Ok(())
}
