use std::ops::Add;
use reqwest::blocking::Client as BlockingClient;
use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct WebhookContent {
    content: String,
}

pub(crate) fn send_discord_message(webhook_url: &str, message: &str) -> bool {
    if webhook_url == "none"{
        eprintln!("Webhook not configured");
        return false
    }
    let client = BlockingClient::new();
    let webhook_content = WebhookContent {
        content: message.to_string().add(format!("\n{}", system_details()).as_str()),
    };

    match client.post(webhook_url)
        .json(&webhook_content)
        .send() {
        Ok(response) => {
            if response.status().is_success() {
                println!("Message sent successfully!");
                true
            } else {
                eprintln!("Failed to send message, status: {}", response.status());
                false
            }
        },
        Err(e) => {
            eprintln!("Error sending message: {}", e);
            false
        }


    }
}

pub(crate) async fn send_discord_message_async(webhook_url: &str, message: &str) -> bool {
    if webhook_url == "none" {
        eprintln!("Webhook not configured");
        return false;
    }

    // Create an asynchronous client
    let client = Client::new();
    let webhook_content = WebhookContent {
        content: format!("{}\n{}", message, system_details()),
    };

    // Send the HTTP POST request asynchronously
    match client.post(webhook_url)
        .json(&webhook_content)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                println!("Message sent successfully!");
                true
            } else {
                eprintln!("Failed to send message, status: {}", response.status());
                false
            }
        }
        Err(e) => {
            eprintln!("Error sending message: {}", e);
            false
        }
    }
}


fn system_details() -> String {
    let info = os_info::get();

    format!(
        "OS: {} {} (Type: {}, Version: {})",
        info.os_type(),
        info.version(),
        info.bitness(),
        info.edition().unwrap_or_default()
    )
}

