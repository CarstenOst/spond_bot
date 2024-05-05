mod structs;
mod accept_event;
mod update_events;
mod get_next_event;
mod accurate_sleep;
mod discord_message;
mod get_member_by_id;


use discord_message::send_discord_message;
use get_member_by_id::get_name_by_id;
use update_events::{update_events};
use get_next_event::get_next_event;
use accurate_sleep::accurate_sleep;
use accept_event::accept_event;
use std::fs::{self, File};
use std::io::prelude::*;
use std::{env, process};
use std::error::Error;
use std::path::Path;
use structs::config::{Config, ConfigSet};

use chrono::Utc;

// DB736DCD48454137B16355ED7AB7AC50
const ACCEPT_URL_BASE: &str = "https://api.spond.com/core/v1/sponds/";



// Function to read and parse the config.json, or create it if it doesn't exist,
// and check for default values
fn read_config() -> Result<Config, Box<dyn Error>> {
    // Find the directory of the current executable
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or("Unable to get executable directory")?;
    println!("{:?}", exe_dir);

    // Set the current directory to the executable's directory
    env::set_current_dir(&exe_dir)?;

    let config_path = "config.toml";

    if !Path::new(config_path).exists() {
        let default_config = r#"
[[config]]
bearer_token = "your_bearer_token_here"
group_id = "your_group_id_here"
user_id = "your_user_id_here"
discord_webhook = "none"
"#;
        let mut file = File::create(config_path)?;
        file.write_all(default_config.as_bytes())?;
        eprintln!("Config file created at '{}'. Please fill in the required information and rerun the program.", config_path);
        process::exit(1);
    }

    let config_str = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;

    Ok(config)
}

fn current_date_time_iso8601() -> String {
    let now = Utc::now();
    let formatted = now.format("%Y-%m-%dT%H:%M:%SZ").to_string(); // ISO 8601 format
    formatted
}

fn update_event_build(config: &ConfigSet) {
    let base_url = format!(
        "https://api.spond.com/core/v1/sponds?includeComments=false&includeHidden=false&addProfileInfo=false&scheduled=true&order=asc&max=20&groupId={}&minEndTimestamp={}",
        config.group_id, current_date_time_iso8601()
    );
    // Assuming update_events sends a request to update events and returns Result<(), Error>
    let _ = update_events(&base_url, &config.bearer_token);
}

fn setup(configs: &Config) {
    if let Some(first_config_set) = configs.config.get(0) {
        update_event_build(first_config_set);
        clear_console();
    } else {
        println!("No configuration sets are available.");
        process::exit(1);
    }
    for config in &configs.config {
        let id = get_next_event("events.json").unwrap().map(|(val, _, _)| val).unwrap().to_string();
        let tmp_url = format!("{}/{}/responses/{}", ACCEPT_URL_BASE, id, config.user_id);

        if !is_config_correct(&tmp_url, &config.bearer_token) {
            eprintln!("Config contains wrong values, update it and restart");
            process::exit(1);
        }
    }
}

fn hello(configs: &Config) {
    for config in &configs.config {
        let user_name = get_name_by_id("response.json", &config.user_id)
            .map(|(first, last)| format!("{} {}", first, last))
            .unwrap_or("No user found".to_string());

        if user_name == String::from("No user found"){
            println!("{}", user_name);
            continue
        }
        println!("Hello there {}!", &user_name);
    }

}


fn main() /*-> Result<(), Box<dyn Error>>*/ {
    clear_console();

    let config: Config = read_config().unwrap();

    setup(&config);

    println!("Starting main script...");
    the_blob_of_logic(&config)

}


fn the_blob_of_logic(configs: &Config) {
    // Retrieve the first configuration if available, and end the process if not
    let first_config = match configs.config.get(0) {
        Some(config) => config,
        None => {
            println!("No configuration sets are available.");
            process::exit(1);
        }
    };

    hello(&configs);

    loop {
        update_event_build(&first_config); // should only happen once per loop
        match get_next_event("events.json") {
            Ok(Some((id, Some(invite_time), header))) => {
                let url = format!("{}/{}/responses/{}", ACCEPT_URL_BASE, id, &first_config.user_id);
                let time_accuracy = accurate_sleep(&invite_time); // Sleep until the event starts
                for config in &configs.config {
                    let user_name = get_name_by_id("response.json", &config.user_id)
                        .map(|(first, last)| format!("{} {}", first, last))
                        .unwrap_or("No user found".to_string());

                    match accept_event(&url, &config.bearer_token) {
                        Ok(_) => {
                            let time = &time_accuracy.num_nanoseconds().unwrap().abs();
                            println!("API request sent for {}\nWith a total of {} nanoseconds time delay\n Congrats {}!", &header, &time, &user_name);
                            send_discord_message(
                                &config.discord_webhook,
                                format!("API request sent for {}\nWith a total of {} nanoseconds time delay\n Congrats {}!", &header, &time, &user_name).as_str());
                        }
                        Err(e) => {
                            println!("Failed to accept event: {}", e);
                            //break; // Exit the loop if accept_event fails
                        }
                    }
                }

            }
            Ok(None) => {
                println!("No more events to process.");
                send_discord_message(&first_config.discord_webhook, "No more events to process");
                break; // Exit the loop if there are no more events
            }
            Err(e) => {
                println!("Error getting next event: {}", e);
                send_discord_message(&first_config.discord_webhook, format!("Error getting next event: {}\nYou should do it manually to still get a spot", e).as_str());
                break; // Exit the loop on error
            }
            _ => {}
        }
    }
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