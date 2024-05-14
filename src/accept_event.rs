use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde_json::json;
use std::error::Error;



pub(crate)fn accept_event(url: &str, bearer_token: &str) -> Result<String, Box<dyn Error>> {
    let payload = json!({
        "accepted": true
    });

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", bearer_token).parse()?);

    let client = reqwest::blocking::Client::new();
    let res = client.put(url)
        .json(&payload)
        .headers(headers)
        .send()?;

    if res.status().is_success() {
        let response_json: serde_json::Value = res.json()?;
        if response_json.get("errorKey").and_then(|k| k.as_str()) == Some("inviteNotSent") {
            return Ok("Invite was not accepted, due to being too fast.".to_string());
        }
        Ok("Success".to_string())
    } else {
        let body = res.text()?;
        Ok(format!("Failed: {}", body))
    }
}





pub(crate) async fn accept_event_async(url: &str, bearer_token: &str) -> Result<String, Box<dyn Error>> {
    let payload = json!({
        "accepted": true
    });

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", bearer_token).parse()?);

    let client = reqwest::Client::new();
    let res = client.put(url)
        .json(&payload)
        .headers(headers)
        .send()
        .await?;

    if res.status().is_success() {
        let response_json: serde_json::Value = res.json().await?;
        if response_json.get("errorKey").and_then(|k| k.as_str()) == Some("inviteNotSent") {
            return Ok("Invite was not accepted, due to being too fast.".to_string());
        }
        Ok("Success".to_string())
    } else {
        let body = res.text().await?;
        Ok(format!("Failed: {}", body))
    }
}
