use crate::startup;
use crate::tests::TestResult;
use crate::tests::common::{cleanup_user, get_test_username, make_auth_request};
use serde_json::json;

pub async fn test_signup_email() -> Result<TestResult, String> {
    let username = get_test_username();
    let payload = json!({
        "username": username.clone(),
        "email": format!("{}@example.com", username),
        "password": "TestPassword123!"
    });

    let timer = startup::create_timer();
    make_auth_request("/auth/signup", &payload, 201).await?;
    let endpoint_time = timer.elapsed();

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
    let endpoint_time = timer.elapsed();

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
    let endpoint_time = timer.elapsed();

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
    let endpoint_time = timer.elapsed();

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

    let login_payload = json!({
        "username_or_email": username.clone(),
        "password": "TestPassword123!"
    });

    let login_body = make_auth_request("/auth/login", &login_payload, 200).await?;
    let user_id = login_body["user"]["id"]
        .as_str()
        .ok_or("Failed to get user ID from login")?
        .to_string();

    let me_payload = json!({
        "username_or_email": username.clone(),
        "password": "TestPassword123!",
        "user_id": user_id.clone(),
        "fields": ["id"]
    });

    make_auth_request("/auth/me", &me_payload, 200).await?;

    let delete_payload = json!({
        "username_or_email": username,
        "password": "TestPassword123!",
        "user_id": user_id
    });

    let timer = startup::create_timer();
    make_auth_request("/auth/delete", &delete_payload, 200).await?;
    let endpoint_time = timer.elapsed();

    Ok(TestResult { endpoint_time })
}

pub async fn test_delete_user_account_username() -> Result<TestResult, String> {
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
    let user_id = login_body["user"]["id"]
        .as_str()
        .ok_or("Failed to get user ID from login")?
        .to_string();

    let me_payload = json!({
        "username_or_email": username.clone(),
        "password": "TestPassword123!",
        "user_id": user_id.clone(),
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
    let endpoint_time = timer.elapsed();

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

    let user_id = login_body["user"]["id"]
        .as_str()
        .ok_or("Failed to get user ID")?
        .to_string();

    let refresh_payload = json!({
        "refresh_token": refresh_token,
        "user_id": user_id
    });

    let timer = startup::create_timer();
    let refresh_body = make_auth_request("/auth/refresh", &refresh_payload, 200).await?;
    let endpoint_time = timer.elapsed();

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

    let user_id = login_body["user"]["id"]
        .as_str()
        .ok_or("Failed to get user ID")?
        .to_string();

    let logout_payload = json!({
        "refresh_token": refresh_token,
        "user_id": user_id
    });

    let timer = startup::create_timer();
    make_auth_request("/auth/logout", &logout_payload, 200).await?;
    let endpoint_time = timer.elapsed();

    let _ = cleanup_user(&username, "TestPassword123!").await;

    Ok(TestResult { endpoint_time })
}

pub async fn test_update_user_profile() -> Result<TestResult, String> {
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

    let new_username = get_test_username();
    let update_payload = json!({
        "user_id": user_id,
        "email": format!("{}@example.com", new_username),
        "username": new_username.clone()
    });

    let timer = startup::create_timer();
    let update_body = make_auth_request("/auth/updateuserprofile", &update_payload, 200).await?;
    let endpoint_time = timer.elapsed();

    if update_body["user"]["username"].as_str() != Some(new_username.as_str()) {
        return Err("Username was not updated correctly".to_string());
    }

    let _ = cleanup_user(&new_username, "TestPassword123!").await;

    Ok(TestResult { endpoint_time })
}