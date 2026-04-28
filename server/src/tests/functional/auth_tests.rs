use crate::startup;
use crate::tests::TestResult;
use serde_json::json;

static CLIENT: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();

#[allow(clippy::redundant_closure)]
fn get_client() -> &'static reqwest::Client {
    CLIENT.get_or_init(|| reqwest::Client::new())
}

async fn make_auth_request(
    endpoint: &str,
    payload: &serde_json::Value,
    expected_status: u16,
) -> Result<serde_json::Value, String> {
    let client = get_client();
    let server_url =
        std::env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

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

fn get_test_username() -> String {
    use rand::RngExt;
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let random: u32 = rand::rng().random_range(0..10000);
    format!("t{}_{}", timestamp % 1000000, random)
}

async fn cleanup_user(username: &str, password: &str) -> Result<(), String> {
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

pub async fn test_signup_email() -> Result<TestResult, String> {
    let username = get_test_username();
    let payload = json!({
        "username": username.clone(),
        "email": format!("{}@example.com", username),
        "password": "TestPassword123!"
    });

    let timer = startup::create_timer();
    make_auth_request("/auth/signup", &payload, 201).await?;
    let endpoint_time = startup::elapsed_ms(timer);

    let _ = cleanup_user(&username, "TestPassword123!").await;

    Ok(TestResult { endpoint_time })
}

pub async fn test_signup_username() -> Result<TestResult, String> {
    let username = get_test_username();
    let payload = json!({
        "username": username.clone(),
        "password": "TestPassword123!"
    });

    let timer = startup::create_timer();
    make_auth_request("/auth/signup", &payload, 201).await?;
    let endpoint_time = startup::elapsed_ms(timer);

    let _ = cleanup_user(&username, "TestPassword123!").await;

    Ok(TestResult { endpoint_time })
}

pub async fn test_login_email() -> Result<TestResult, String> {
    let username = get_test_username();
    let signup_payload = json!({
        "username": username.clone(),
        "email": format!("{}@example.com", username),
        "password": "TestPassword123!"
    });

    make_auth_request("/auth/signup", &signup_payload, 201).await?;

    let login_payload = json!({
        "username_or_email": username.clone(),
        "password": "TestPassword123!"
    });

    let timer = startup::create_timer();
    make_auth_request("/auth/login", &login_payload, 200).await?;
    let endpoint_time = startup::elapsed_ms(timer);

    let _ = cleanup_user(&username, "TestPassword123!").await;

    Ok(TestResult { endpoint_time })
}

pub async fn test_login_username() -> Result<TestResult, String> {
    let username = get_test_username();
    let signup_payload = json!({
        "username": username.clone(),
        "password": "TestPassword123!"
    });

    make_auth_request("/auth/signup", &signup_payload, 201).await?;

    let login_payload = json!({
        "username_or_email": username.clone(),
        "password": "TestPassword123!"
    });

    let timer = startup::create_timer();
    make_auth_request("/auth/login", &login_payload, 200).await?;
    let endpoint_time = startup::elapsed_ms(timer);

    let _ = cleanup_user(&username, "TestPassword123!").await;

    Ok(TestResult { endpoint_time })
}

pub async fn test_delete_user_account_email() -> Result<TestResult, String> {
    let username = get_test_username();
    let signup_payload = json!({
        "username": username.clone(),
        "email": format!("{}@example.com", username),
        "password": "TestPassword123!"
    });

    make_auth_request("/auth/signup", &signup_payload, 201).await?;

    let me_payload = json!({
        "username_or_email": username.clone(),
        "password": "TestPassword123!",
        "fields": ["id"]
    });

    let me_body = make_auth_request("/auth/me", &me_payload, 200).await?;

    let user_id = me_body["data"]["id"]
        .as_str()
        .ok_or("Failed to get user ID")?
        .to_string();

    let delete_payload = json!({
        "username_or_email": username,
        "password": "TestPassword123!",
        "user_id": user_id
    });

    let timer = startup::create_timer();
    make_auth_request("/auth/delete", &delete_payload, 200).await?;
    let endpoint_time = startup::elapsed_ms(timer);

    Ok(TestResult { endpoint_time })
}

pub async fn test_delete_user_account_username() -> Result<TestResult, String> {
    let username = get_test_username();
    let signup_payload = json!({
        "username": username.clone(),
        "password": "TestPassword123!"
    });

    make_auth_request("/auth/signup", &signup_payload, 201).await?;

    let me_payload = json!({
        "username_or_email": username.clone(),
        "password": "TestPassword123!",
        "fields": ["id"]
    });

    let me_body = make_auth_request("/auth/me", &me_payload, 200).await?;

    let user_id = me_body["data"]["id"]
        .as_str()
        .ok_or("Failed to get user ID")?
        .to_string();

    let delete_payload = json!({
        "username_or_email": username,
        "password": "TestPassword123!",
        "user_id": user_id
    });

    let timer = startup::create_timer();
    make_auth_request("/auth/delete", &delete_payload, 200).await?;
    let endpoint_time = startup::elapsed_ms(timer);

    Ok(TestResult { endpoint_time })
}

pub async fn test_refresh_token() -> Result<TestResult, String> {
    let username = get_test_username();
    let signup_payload = json!({
        "username": username.clone(),
        "password": "TestPassword123!"
    });

    make_auth_request("/auth/signup", &signup_payload, 201).await?;

    let login_payload = json!({
        "username_or_email": username.clone(),
        "password": "TestPassword123!"
    });

    let login_body = make_auth_request("/auth/login", &login_payload, 200).await?;

    let refresh_token = login_body["tokens"]["refresh_token"]
        .as_str()
        .ok_or("Failed to get refresh token")?
        .to_string();

    let refresh_payload = json!({
        "refresh_token": refresh_token
    });

    let timer = startup::create_timer();
    let refresh_body = make_auth_request("/auth/refresh", &refresh_payload, 200).await?;
    let endpoint_time = startup::elapsed_ms(timer);

    if refresh_body["tokens"]["access_token"].as_str().is_none() {
        return Err("Failed to get new access token".to_string());
    }

    let _ = cleanup_user(&username, "TestPassword123!").await;

    Ok(TestResult { endpoint_time })
}

pub async fn test_logout() -> Result<TestResult, String> {
    let username = get_test_username();
    let signup_payload = json!({
        "username": username.clone(),
        "password": "TestPassword123!"
    });

    make_auth_request("/auth/signup", &signup_payload, 201).await?;

    let login_payload = json!({
        "username_or_email": username.clone(),
        "password": "TestPassword123!"
    });

    let login_body = make_auth_request("/auth/login", &login_payload, 200).await?;

    let refresh_token = login_body["tokens"]["refresh_token"]
        .as_str()
        .ok_or("Failed to get refresh token")?
        .to_string();

    let logout_payload = json!({
        "refresh_token": refresh_token
    });

    let timer = startup::create_timer();
    make_auth_request("/auth/logout", &logout_payload, 200).await?;
    let endpoint_time = startup::elapsed_ms(timer);

    let _ = cleanup_user(&username, "TestPassword123!").await;

    Ok(TestResult { endpoint_time })
}
