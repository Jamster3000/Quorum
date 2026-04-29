use crate::utility::config::Config;
use serde_json::json;

static CLIENT: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();

#[allow(clippy::redundant_closure)]
fn get_client() -> &'static reqwest::Client {
    CLIENT.get_or_init(|| reqwest::Client::new())
}

pub async fn make_auth_request(
    endpoint: &str,
    payload: &serde_json::Value,
    expected_status: u16,
) -> Result<serde_json::Value, String> {
    let client = get_client();
    let server_url = Config::get().server_url.clone();

    //Make a request to an endpoint and return the response
    let response = client
        .post(format!("{}{}", server_url, endpoint))
        .json(payload)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = response.status().as_u16();
    let body: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if status != expected_status {
        let error_msg = body["message"].as_str().unwrap_or("Unknown error");
        return Err(format!(
            "Expected {}, got {} - {}",
            expected_status, status, error_msg
        ));
    }

    if body["success"] != true {
        return Err(format!("Request failed: {}", body["message"]));
    }

    Ok(body)
}

pub async fn make_auth_request_raw(
    endpoint: &str,
    payload: &serde_json::Value,
    expected_status: u16,
) -> Result<serde_json::Value, String> {
    let client = get_client();
    let server_url = Config::get().server_url.clone();

    let response = client
        .post(format!("{}{}", server_url, endpoint))
        .json(payload)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = response.status().as_u16();
    let body: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if status != expected_status {
        let error_msg = body["message"].as_str().unwrap_or("Unknown error");
        return Err(format!(
            "Expected {}, got {} - {}",
            expected_status, status, error_msg
        ));
    }

    Ok(body)
}

pub fn get_test_username() -> String {
    use rand::RngExt;
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let random: u32 = rand::rng().random_range(0..10000);
    format!("t{}_{}", timestamp % 1000000, random)
}

pub async fn cleanup_user(username: &str, password: &str) -> Result<(), String> {
    let me_payload = json!({
        "username_or_email": username,
        "password": password,
        "fields": ["id"]
    });

    let me_body = match make_auth_request("/auth/me", &me_payload, 200).await {
        Ok(body) => body,
        Err(_) => return Ok(()),
    };

    let user_id = me_body["data"]["id"]
        .as_str()
        .ok_or("Failed to get user ID")?
        .to_string();

    let delete_payload = json!({
        "username_or_email": username,
        "password": password,
        "user_id": user_id
    });

    make_auth_request("/auth/delete", &delete_payload, 200).await?;
    Ok(())
}