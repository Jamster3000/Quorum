use crate::startup;
use crate::tests::RobustnessTestResult;
use crate::tests::common::{cleanup_user, get_test_username, make_auth_request_raw};
use serde_json::json;

/// Test signup with very short username (1 character)
pub async fn test_signup_short_username() -> Result<RobustnessTestResult, String> {
    let timer = startup::create_timer();
    make_auth_request_raw("/auth/signup", &json!({
        "username": "a",
        "password": "TestPassword123!"
    }), 500).await?;
    let endpoint_time = timer.elapsed();

    Ok(RobustnessTestResult { endpoint_time })
}

/// Test signup with very long username (256+ characters)
pub async fn test_signup_long_username() -> Result<RobustnessTestResult, String> {
    let timer = startup::create_timer();
    make_auth_request_raw("/auth/signup", &json!({
        "username": "a".repeat(256),
        "password": "TestPassword123!"
    }), 500).await?;
    let endpoint_time = timer.elapsed();

    Ok(RobustnessTestResult { endpoint_time })
}

/// Test signup with empty password
pub async fn test_signup_empty_password() -> Result<RobustnessTestResult, String> {
    let timer = startup::create_timer();
    make_auth_request_raw("/auth/signup", &json!({
        "username": get_test_username(),
        "password": ""
    }), 400).await?;
    let endpoint_time = timer.elapsed();

    Ok(RobustnessTestResult { endpoint_time })
}

/// Test signup with very short password (1 character)
pub async fn test_signup_short_password() -> Result<RobustnessTestResult, String> {
    let timer = startup::create_timer();
    make_auth_request_raw("/auth/signup", &json!({
        "username": get_test_username(),
        "password": "a"
    }), 400).await?;
    let endpoint_time = timer.elapsed();

    Ok(RobustnessTestResult { endpoint_time })
}

/// Test signup with very long password (1000+ characters)
pub async fn test_signup_long_password() -> Result<RobustnessTestResult, String> {
    let timer = startup::create_timer();
    make_auth_request_raw("/auth/signup", &json!({
        "username": get_test_username(),
        "password": "a".repeat(1000)
    }), 400).await?;
    let endpoint_time = timer.elapsed();

    Ok(RobustnessTestResult { endpoint_time })
}

/// Test signup with invalid email format
pub async fn test_signup_invalid_email() -> Result<RobustnessTestResult, String> {
    let timer = startup::create_timer();
    make_auth_request_raw("/auth/signup", &json!({
        "username": get_test_username(),
        "email": "not-an-email",
        "password": "TestPassword123!"
    }), 500).await?;
    let endpoint_time = timer.elapsed();

    Ok(RobustnessTestResult { endpoint_time })
}

/// Test signup with duplicate username
pub async fn test_signup_duplicate_username() -> Result<RobustnessTestResult, String> {
    let username = get_test_username();

    let login_body = make_auth_request_raw("/auth/signup", &json!({
        "username": username,
        "password": "TestPassword123!"
    }), 201).await?;
    let user_id = login_body["user"]["id"].as_str().ok_or("Failed to get user ID")?.to_string();

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/signup", &json!({
        "username": username,
        "password": "DifferentPassword123!"
    }), 400).await?;
    let endpoint_time = timer.elapsed();

    let _ = cleanup_user(&username, "TestPassword123!", &user_id).await;
    Ok(RobustnessTestResult { endpoint_time })
}
/// Test login with wrong password
pub async fn test_login_wrong_password() -> Result<RobustnessTestResult, String> {
    let username = get_test_username();

    make_auth_request_raw("/auth/signup", &json!({
        "username": username,
        "password": "CorrectPassword123!"
    }), 201).await?;

    let login_body = make_auth_request_raw("/auth/login", &json!({
        "username_or_email": username,
        "password": "CorrectPassword123!"
    }), 200).await?;
    let user_id = login_body["user"]["id"].as_str().ok_or("Failed to get user ID")?.to_string();

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/login", &json!({
        "username_or_email": username,
        "password": "WrongPassword123!"
    }), 401).await?;
    let endpoint_time = timer.elapsed();

    let _ = cleanup_user(&username, "CorrectPassword123!", &user_id).await;
    Ok(RobustnessTestResult { endpoint_time })
}

/// Test login with nonexistent user
pub async fn test_login_nonexistent_user() -> Result<RobustnessTestResult, String> {
    let timer = startup::create_timer();
    make_auth_request_raw("/auth/login", &json!({
        "username_or_email": "nonexistent_user_12345",
        "password": "TestPassword123!"
    }), 401).await?;
    let endpoint_time = timer.elapsed();

    Ok(RobustnessTestResult { endpoint_time })
}

/// Test login with empty username
pub async fn test_login_empty_username() -> Result<RobustnessTestResult, String> {
    let timer = startup::create_timer();
    make_auth_request_raw("/auth/login", &json!({
        "username_or_email": "",
        "password": "TestPassword123!"
    }), 401).await?;
    let endpoint_time = timer.elapsed();

    Ok(RobustnessTestResult { endpoint_time })
}

/// Test refresh token with invalid token format
pub async fn test_refresh_invalid_token() -> Result<RobustnessTestResult, String> {
    let username = get_test_username();

    make_auth_request_raw("/auth/signup", &json!({
        "username": username,
        "password": "TestPassword123!"
    }), 201).await?;

    let login_body = make_auth_request_raw("/auth/login", &json!({
        "username_or_email": username,
        "password": "TestPassword123!"
    }), 200).await?;
    let user_id = login_body["user"]["id"].as_str().ok_or("Failed to get user ID")?.to_string();

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/refresh", &json!({
        "user_id": user_id,
        "refresh_token": "not-a-valid-jwt-token"
    }), 401).await?;
    let endpoint_time = timer.elapsed();

    let _ = cleanup_user(&username, "TestPassword123!", &user_id).await;
    Ok(RobustnessTestResult { endpoint_time })
}

/// Test refresh token with empty token
pub async fn test_refresh_empty_token() -> Result<RobustnessTestResult, String> {
    let username = get_test_username();

    make_auth_request_raw("/auth/signup", &json!({
        "username": username,
        "password": "TestPassword123!"
    }), 201).await?;

    let login_body = make_auth_request_raw("/auth/login", &json!({
        "username_or_email": username,
        "password": "TestPassword123!"
    }), 200).await?;
    let user_id = login_body["user"]["id"].as_str().ok_or("Failed to get user ID")?.to_string();

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/refresh", &json!({
        "user_id": user_id,
        "refresh_token": ""
    }), 401).await?;
    let endpoint_time = timer.elapsed();

    let _ = cleanup_user(&username, "TestPassword123!", &user_id).await;
    Ok(RobustnessTestResult { endpoint_time })
}

/// Test delete with wrong password
pub async fn test_delete_wrong_password() -> Result<RobustnessTestResult, String> {
    let username = get_test_username();

    make_auth_request_raw("/auth/signup", &json!({
        "username": username,
        "password": "CorrectPassword123!"
    }), 201).await?;

    let login_body = make_auth_request_raw("/auth/login", &json!({
        "username_or_email": username,
        "password": "CorrectPassword123!"
    }), 200).await?;
    let user_id = login_body["user"]["id"].as_str().ok_or("Failed to get user ID")?.to_string();

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/delete", &json!({
        "username_or_email": username,
        "password": "WrongPassword123!",
        "user_id": user_id
    }), 401).await?;
    let endpoint_time = timer.elapsed();

    let _ = cleanup_user(&username, "CorrectPassword123!", &user_id).await;
    Ok(RobustnessTestResult { endpoint_time })
}

/// Test get user data with wrong password
pub async fn test_get_user_data_wrong_password() -> Result<RobustnessTestResult, String> {
    let username = get_test_username();

    make_auth_request_raw("/auth/signup", &json!({
        "username": username,
        "password": "CorrectPassword123!"
    }), 201).await?;

    let login_body = make_auth_request_raw("/auth/login", &json!({
        "username_or_email": username,
        "password": "CorrectPassword123!"
    }), 200).await?;
    let user_id = login_body["user"]["id"].as_str().ok_or("Failed to get user ID")?.to_string();

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/me", &json!({
        "username_or_email": username,
        "password": "WrongPassword123!",
        "user_id": user_id,
        "fields": ["username"]
    }), 401).await?;
    let endpoint_time = timer.elapsed();

    let _ = cleanup_user(&username, "CorrectPassword123!", &user_id).await;
    Ok(RobustnessTestResult { endpoint_time })
}

/// Test logout with invalid refresh token
pub async fn test_logout_invalid_token() -> Result<RobustnessTestResult, String> {
    let username = get_test_username();

    make_auth_request_raw("/auth/signup", &json!({
        "username": username,
        "password": "TestPassword123!"
    }), 201).await?;

    let login_body = make_auth_request_raw("/auth/login", &json!({
        "username_or_email": username,
        "password": "TestPassword123!"
    }), 200).await?;
    let user_id = login_body["user"]["id"].as_str().ok_or("Failed to get user ID")?.to_string();

    let timer = startup::create_timer();
    make_auth_request_raw("/auth/logout", &json!({
        "user_id": user_id,
        "refresh_token": "not-a-valid-token"
    }), 401).await?;
    let endpoint_time = timer.elapsed();

    let _ = cleanup_user(&username, "TestPassword123!", &user_id).await;
    Ok(RobustnessTestResult { endpoint_time })
}