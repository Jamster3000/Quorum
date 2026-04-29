use crate::tests::common::{make_auth_request_raw, get_test_username};
use crate::tests::RobustnessTestResult;
use serde_json::json;
use crate::startup;

/// Test signup with very short username (1 character)
pub async fn test_signup_short_username() -> Result<RobustnessTestResult, String> {
    let payload = json!({
        "username": "a",
        "password": "TestPassword123!"
    });

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/signup", &payload, 500).await?;
    let endpoint_time = startup::elapsed_ms(timer);
    Ok(RobustnessTestResult { endpoint_time })
}

/// Test signup with very long username (256+ characters)
pub async fn test_signup_long_username() -> Result<RobustnessTestResult, String> {
    let long_username = "a".repeat(256);
    let payload = json!({
        "username": long_username,
        "password": "TestPassword123!"
    });

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/signup", &payload, 500).await?;
    let endpoint_time = startup::elapsed_ms(timer);
    Ok(RobustnessTestResult { endpoint_time })
}

/// Test signup with empty password
pub async fn test_signup_empty_password() -> Result<RobustnessTestResult, String> {
    let username = get_test_username();
    let payload = json!({
        "username": username,
        "password": ""
    });

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/signup", &payload, 400).await?;
    let endpoint_time = startup::elapsed_ms(timer);
    Ok(RobustnessTestResult { endpoint_time })
}

/// Test signup with very short password (1 character)
pub async fn test_signup_short_password() -> Result<RobustnessTestResult, String> {
    let username = get_test_username();
    let payload = json!({
        "username": username,
        "password": "a"
    });

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/signup", &payload, 400).await?;
    let endpoint_time = startup::elapsed_ms(timer);
    Ok(RobustnessTestResult { endpoint_time })
}

/// Test signup with very long password (1000+ characters)
pub async fn test_signup_long_password() -> Result<RobustnessTestResult, String> {
    let username = get_test_username();
    let long_password = "a".repeat(1000);
    let payload = json!({
        "username": username,
        "password": long_password
    });

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/signup", &payload, 400).await?;
    let endpoint_time = startup::elapsed_ms(timer);
    Ok(RobustnessTestResult { endpoint_time})
}

/// Test signup with invalid email format
pub async fn test_signup_invalid_email() -> Result<RobustnessTestResult, String> {
    let username = get_test_username();
    let payload = json!({
        "username": username,
        "email": "not-an-email",
        "password": "TestPassword123!"
    });

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/signup", &payload, 500).await?;
    let endpoint_time = startup::elapsed_ms(timer);
    Ok(RobustnessTestResult { endpoint_time})
}

/// Test signup with duplicate username
pub async fn test_signup_duplicate_username() -> Result<RobustnessTestResult, String> {
    let username = get_test_username();
    let payload = json!({
        "username": username.clone(),
        "password": "TestPassword123!"
    });

    make_auth_request_raw("/auth/signup", &payload, 201).await?;

    let duplicate_payload = json!({
        "username": username.clone(),
        "password": "DifferentPassword123!"
    });

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/signup", &duplicate_payload, 400).await?;
    let endpoint_time = startup::elapsed_ms(timer);
    Ok(RobustnessTestResult { endpoint_time})
}

/// Test login with wrong password
pub async fn test_login_wrong_password() -> Result<RobustnessTestResult, String> {
    let username = get_test_username();
    let signup_payload = json!({
        "username": username.clone(),
        "password": "CorrectPassword123!"
    });

    make_auth_request_raw("/auth/signup", &signup_payload, 201).await?;

    let login_payload = json!({
        "username_or_email": username.clone(),
        "password": "WrongPassword123!"
    });

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/login", &login_payload, 401).await?;
    let endpoint_time = startup::elapsed_ms(timer);
    Ok(RobustnessTestResult { endpoint_time})
}

/// Test login with nonexistent user
pub async fn test_login_nonexistent_user() -> Result<RobustnessTestResult, String> {
    let payload = json!({
        "username_or_email": "nonexistent_user_12345",
        "password": "TestPassword123!"
    });

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/login", &payload, 401).await?;
    let endpoint_time = startup::elapsed_ms(timer);
    Ok(RobustnessTestResult { endpoint_time})
}

/// Test login with empty username
pub async fn test_login_empty_username() -> Result<RobustnessTestResult, String> {
    let payload = json!({
        "username_or_email": "",
        "password": "TestPassword123!"
    });

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/login", &payload, 401).await?;
    let endpoint_time = startup::elapsed_ms(timer);
    Ok(RobustnessTestResult { endpoint_time})
}

/// Test refresh token with invalid token format
pub async fn test_refresh_invalid_token() -> Result<RobustnessTestResult, String> {
    let payload = json!({
        "refresh_token": "not-a-valid-jwt-token"
    });

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/refresh", &payload, 401).await?;
    let endpoint_time = startup::elapsed_ms(timer);
    Ok(RobustnessTestResult { endpoint_time})
}

/// Test refresh token with empty token
pub async fn test_refresh_empty_token() -> Result<RobustnessTestResult, String> {
    let payload = json!({
        "refresh_token": ""
    });

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/refresh", &payload, 401).await?;
    let endpoint_time = startup::elapsed_ms(timer);
    Ok(RobustnessTestResult { endpoint_time})
}

/// Test delete with wrong password
pub async fn test_delete_wrong_password() -> Result<RobustnessTestResult, String> {
    let username = get_test_username();
    let signup_payload = json!({
        "username": username.clone(),
        "password": "CorrectPassword123!"
    });

    make_auth_request_raw("/auth/signup", &signup_payload, 201).await?;

    let me_payload = json!({
        "username_or_email": username.clone(),
        "password": "CorrectPassword123!",
        "fields": ["id"]
    });

    let me_body = make_auth_request_raw("/auth/me", &me_payload, 200).await?;
    let user_id = me_body["data"]["id"]
        .as_str()
        .ok_or("Failed to get user ID")?
        .to_string();

    let delete_payload = json!({
        "username_or_email": username.clone(),
        "password": "WrongPassword123!",
        "user_id": user_id
    });

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/delete", &delete_payload, 401).await?;
    let endpoint_time = startup::elapsed_ms(timer);
    Ok(RobustnessTestResult { endpoint_time})
}

/// Test get user data with wrong password
pub async fn test_get_user_data_wrong_password() -> Result<RobustnessTestResult, String> {
    let username = get_test_username();
    let signup_payload = json!({
        "username": username.clone(),
        "password": "CorrectPassword123!"
    });

    make_auth_request_raw("/auth/signup", &signup_payload, 201).await?;

    let payload = json!({
        "username_or_email": username.clone(),
        "password": "WrongPassword123!",
        "fields": ["username"]
    });

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/me", &payload, 401).await?;
    let endpoint_time = startup::elapsed_ms(timer);
    Ok(RobustnessTestResult { endpoint_time})
}

/// Test logout with invalid refresh token
pub async fn test_logout_invalid_token() -> Result<RobustnessTestResult, String> {
    let payload = json!({
        "refresh_token": "not-a-valid-token"
    });

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/logout", &payload, 401).await?;
    let endpoint_time = startup::elapsed_ms(timer);
    Ok(RobustnessTestResult { endpoint_time})
}