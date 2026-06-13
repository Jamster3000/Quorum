use crate::startup;
use crate::tests::TestResult;
use crate::tests::common::{cleanup_user, get_test_username, make_auth_request, create_test_user};
use serde_json::json;

pub async fn test_signup_email() -> Result<TestResult, String> {
    let (_, _, _, timings) = create_test_user("TestPassword123!", true, true).await?;
    Ok(TestResult { endpoint_time: timings.signup })
}

pub async fn test_signup_username() -> Result<TestResult, String> {
    let (_, _, _, timings) = create_test_user("TestPassword123!", false, true).await?;
    Ok(TestResult { endpoint_time: timings.signup })
}

pub async fn test_login_email() -> Result<TestResult, String> {
    let (_, _, _, timings) = create_test_user("TestPassword123!", true, true).await?;
    Ok(TestResult { endpoint_time: timings.login })
}

pub async fn test_login_username() -> Result<TestResult, String> {
    let (_, _, _, timings) = create_test_user("TestPassword123!", false, true).await?;
    Ok(TestResult { endpoint_time: timings.login })
}

pub async fn test_delete_user_account_email() -> Result<TestResult, String> {
    let (username, password, user_id, _) = create_test_user("TestPassword123!", true, false).await?;

    make_auth_request("/auth/me", &json!({
        "username_or_email": username,
        "password": password,
        "user_id": user_id,
        "fields": ["id"]
    }), 200).await?;

    let timer = startup::create_timer();
    make_auth_request("/auth/delete", &json!({
        "username_or_email": username,
        "password": password,
        "user_id": user_id
    }), 200).await?;
    let endpoint_time = timer.elapsed();

    Ok(TestResult { endpoint_time })
}

pub async fn test_delete_user_account_username() -> Result<TestResult, String> {
    let (username, password, user_id, _) = create_test_user("TestPassword123!", false, false).await?;

    let timer = startup::create_timer();
    make_auth_request("/auth/delete", &json!({
        "username_or_email": username,
        "password": password,
        "user_id": user_id
    }), 200).await?;
    let endpoint_time = timer.elapsed();

    Ok(TestResult { endpoint_time })
}

pub async fn test_refresh_token() -> Result<TestResult, String> {
    let (username, password, user_id, _) = create_test_user("TestPassword123!", false, false).await?;

    let login_body = make_auth_request("/auth/login", &json!({
        "username_or_email": username,
        "password": password
    }), 200).await?;

    let refresh_token = login_body["tokens"]["refresh_token"]
        .as_str()
        .ok_or("Failed to get refresh token")?
        .to_string();

    let timer = startup::create_timer();
    let refresh_body = make_auth_request("/auth/refresh", &json!({
        "refresh_token": refresh_token,
        "user_id": user_id
    }), 200).await?;
    let endpoint_time = timer.elapsed();

    if refresh_body["tokens"]["access_token"].as_str().is_none() {
        return Err("Failed to get new access token".to_string());
    }

    let _ = cleanup_user(&username, &password).await;
    Ok(TestResult { endpoint_time })
}

pub async fn test_logout() -> Result<TestResult, String> {
    let (username, password, user_id, _) = create_test_user("TestPassword123!", false, false).await?;

    let login_body = make_auth_request("/auth/login", &json!({
        "username_or_email": username,
        "password": password
    }), 200).await?;

    let refresh_token = login_body["tokens"]["refresh_token"]
        .as_str()
        .ok_or("Failed to get refresh token")?
        .to_string();

    let timer = startup::create_timer();
    make_auth_request("/auth/logout", &json!({
        "refresh_token": refresh_token,
        "user_id": user_id
    }), 200).await?;
    let endpoint_time = timer.elapsed();

    let _ = cleanup_user(&username, &password).await;
    Ok(TestResult { endpoint_time })
}

pub async fn test_update_user_profile() -> Result<TestResult, String> {
    let (_, password, user_id, _) = create_test_user("TestPassword123!", false, false).await?;

    let new_username = get_test_username();

    let timer = startup::create_timer();
    make_auth_request("/auth/updateuserprofile", &json!({
        "user_id": user_id,
        "username": new_username
    }), 200).await?;
    let endpoint_time = timer.elapsed();

    let _ = cleanup_user(&new_username, &password).await;
    Ok(TestResult { endpoint_time })
}