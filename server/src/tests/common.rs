use crate::startup;
use crate::utility::config::Config;
use serde_json::json;
use std::time::Duration;

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

pub async fn cleanup_user(username: &str, password: &str, user_id: &str) -> Result<(), String> {
    make_auth_request(
        "/auth/delete",
        &json!({
            "username_or_email": username,
            "password": password,
            "user_id": user_id
        }),
        200,
    )
    .await?;
    Ok(())
}

pub struct TestUserTimings {
    pub signup: Duration,
    pub login: Duration,
}

pub async fn create_test_user(
    password: &str,
    email: bool,
    clean_up: bool,
) -> Result<(String, String, String, TestUserTimings), String> {
    let username = get_test_username();

    let signup_timer = startup::create_timer();
    let mut payload = json!({
        "username": username,
        "password": password
    });
    if email {
        payload["email"] = json!(format!("{}@example.com", username));
    }
    make_auth_request("/auth/signup", &payload, 201).await?;
    let signup_time = signup_timer.elapsed();

    let login_timer = startup::create_timer();
    let login_body = make_auth_request(
        "/auth/login",
        &json!({
            "username_or_email": username,
            "password": password
        }),
        200,
    )
    .await?;
    let login_time = login_timer.elapsed();

    let user_id = login_body["user"]["id"]
        .as_str()
        .ok_or("Failed to get user ID")?
        .to_string();

    if clean_up {
        let _ = cleanup_user(&username, password, &user_id).await;
    }

    Ok((
        username,
        password.to_string(),
        user_id,
        TestUserTimings {
            signup: signup_time,
            login: login_time,
        },
    ))
}
