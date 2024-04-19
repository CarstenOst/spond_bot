mod accept_event;
mod update_events;
mod get_next_event;
mod accurate_sleep;
mod discord_message;
mod get_member_by_id;

use get_member_by_id::get_name_by_id;
use discord_message::send_discord_message;
use update_events::{update_events};
use get_next_event::get_next_event;
use accurate_sleep::accurate_sleep;
use accept_event::accept_event;
use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use std::{env, process};
use serde::Deserialize;

use chrono::Utc;

// DB736DCD48454137B16355ED7AB7AC50
const ACCEPT_URL_BASE: &str = "https://api.spond.com/core/v1/sponds/";

#[derive(Deserialize)]
struct Config {
    bearer_token: String,
    group_id: String,
    user_id: String,
    discord_webhook: String,
}

// Function to read and parse the config.json, or create it if it doesn't exist,
// and check for default values
fn read_config() -> Result<Config, Box<dyn Error>> {

    // Find the directory of the current executable
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or("Unable to get executable directory")?;

    println!("{:?}", exe_dir);

    // Set the current directory to the executable's directory
    env::set_current_dir(&exe_dir)?;

    let config_path = "config.json";

    if !Path::new(config_path).exists() {
        let default_config = r#"{
    "bearer_token": "your_bearer_token_here",
    "group_id": "your_group_id_here",
    "user_id": "your_user_id_here",
    "discord_webhook": "none"
}"#;
        let mut file = File::create(config_path)?;
        file.write_all(default_config.as_bytes())?;
        eprintln!("Config file created at '{}'. Please fill in the required information and rerun the program.", config_path);
        process::exit(1);
    }

    let config_str = fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&config_str)?;

    // Validate config values and inform the user about defaults that need to be updated
    let mut is_config_changed = true;

    if config.bearer_token == "your_bearer_token_here" {
        eprintln!("Please update the 'BEARER_TOKEN' in the config file.");
        is_config_changed = false;
    }
    if config.group_id == "your_group_id_here" {
        eprintln!("Please update the 'GROUP_ID' in the config file.");
        is_config_changed = false;
    }
    if config.user_id == "your_user_id_here" {
        eprintln!("Please update the 'USER_ID' in the config file.");
        is_config_changed = false;
    }

    if !is_config_changed {
        process::exit(1);
    }

    Ok(config)
}

fn current_date_time_iso8601() -> String {
    let now = Utc::now();
    let formatted = now.format("%Y-%m-%dT%H:%M:%SZ").to_string(); // ISO 8601 format
    formatted
}

fn update_event_build(config: &Config) {
    // Assume current_date_time_iso8601() returns the current date-time in ISO 8601 format as a String
    let base_url = format!(
        "https://api.spond.com/core/v1/sponds?includeComments=false&includeHidden=false&addProfileInfo=false&scheduled=true&order=asc&max=20&groupId={}&minEndTimestamp={}",
        config.group_id, current_date_time_iso8601()
    );

    // Assuming update_events sends a request to update events and returns Result<(), Error>
    let _ = update_events(&base_url, &config.bearer_token);
}

fn setup(config: &Config) {
    update_event_build(config);
    let id = get_next_event("events.json").unwrap().map(|(val, _, _)| val).unwrap().to_string();
    let tmp_url = format!("{}/{}/responses/{}", ACCEPT_URL_BASE, id, config.user_id);

    if !is_config_correct(&tmp_url, &config.bearer_token) {
        eprintln!("Config contains wrong values, update it and restart");
        process::exit(1);
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = read_config().unwrap();
    setup(&config);
    clear_console();
    let user_name = get_name_by_id("response.json", &config.user_id)
        .map(|(first, last)| format!("{} {}", first, last))
        .unwrap_or("No user found".to_string());

    println!("Hello there {}!", &user_name);


    loop {
        update_event_build(&config);

        match get_next_event("events.json") {
            Ok(Some((id, Some(invite_time), header))) => {
                let url = format!("{}/{}/responses/{}", ACCEPT_URL_BASE, id, &config.user_id);
                let time_accuracy = accurate_sleep(&invite_time);
                match accept_event(&url, &config.bearer_token) {
                    Ok(_) => {
                        let time = &time_accuracy.num_nanoseconds().unwrap().abs();
                        println!("API request sent for {}\nWith a total of {} nanoseconds time delay\n Congrats {}!", &header, &time, &user_name);
                        send_discord_message(
                            &config.discord_webhook,
                            format!("API request sent for {}\nWith a total of {} nanoseconds time delay\n Congrats {}!", &header, &time, &user_name).as_str());
                    } ,
                    Err(e) => {
                        println!("Failed to accept event: {}", e);
                        //break; // Exit the loop if accept_event fails
                    }
                }

            }
            Ok(None) => {
                println!("No more events to process.");
                send_discord_message(&config.discord_webhook, "No more events to process");
                break; // Exit the loop if there are no more events
            }
            Err(e) => {
                println!("Error getting next event: {}", e);
                send_discord_message(&config.discord_webhook, format!("Error getting next event: {}\n {} you should do it manually to still get a spot", e, user_name).as_str());
                break; // Exit the loop on error
            }
            _ => {}
        }
    }

    Ok(())

}


fn is_config_correct(url: &str, bt: &str) -> bool {
    match accept_event(url, bt) {
        Ok(response) => {
            if response.contains("Invitation has not been sent out") {
                // Handle the specific case where the invitation has not been sent out
                println!("Your config is correct!");
                true
                // Optionally, you can return an error or perform other actions here
            } else if response.contains("Failed") {
                // Handle the general success case
                eprintln!(" {}", response);
                false
            } else {
                true
            }
        }
        Err(e) => {
            // Handle errors
            eprintln!("Error checking your config: {}", e);
            false
        }
    }
}


fn clear_console() {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    }

    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new("clear")
            .status()
            .unwrap();
    }
}