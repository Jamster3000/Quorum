use crate::common::make_auth_request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::Emitter;

#[derive(Serialize)]
pub struct AuthSuccess {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
    pub username: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignupSuccess {
    pub message: String,
    pub account_has_email: bool,
}

#[derive(Deserialize)]
pub struct SignupPayload {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username_or_email: String,
    pub password: String,
}

#[tauri::command]
pub async fn signup(
    app: tauri::AppHandle,
    payload: SignupPayload,
) -> Result<SignupSuccess, String> {
    if payload.username.is_empty() {
        return Err("Username must be at least 1 character.".to_string());
    }

    if payload.username.len() > 18 {
        return Err("Username must be under 32 characters.".to_string());
    }

    if let Some(ref email) = payload.email {
        if !email.is_empty() && !email.contains('@') {
            return Err("Please enter a valid email address.".to_string());
        }
    }

    if payload.password.is_empty() {
        return Err("Password cannot be empty.".to_string());
    }

    if payload.password.len() < 8 {
        return Err("Password must be at least 8 characters.".to_string());
    }

    if payload.password.len() > 128 {
        return Err("Password must be under 128 characters.".to_string());
    }

    if payload.password != payload.confirm_password {
        return Err("Passwords do not match.".to_string());
    }

    let has_email = payload
    .email
    .as_ref()
    .is_some_and(|email| !email.trim().is_empty());

    let new_payload = if has_email {
        json!({
            "username": &payload.username,
            "email": payload.email.as_deref().map(str::trim),
              "password": &payload.password,
        })
    } else {
        json!({
            "username": &payload.username,
            "password": &payload.password,
        })
    };

    make_auth_request(&app, "/auth/signup", &new_payload, 201).await?;

    Ok(SignupSuccess {
        message: "Account created successfully.".to_string(),
        account_has_email: has_email,
    })
}

#[tauri::command]
pub async fn login(app: tauri::AppHandle, payload: LoginPayload) -> Result<AuthSuccess, String> {
    let request_payload = json!({
        "username_or_email": payload.username_or_email,
        "password": payload.password,
    });

    let auth_response = make_auth_request(&app, "/auth/login", &request_payload, 200).await?;

    if !auth_response["success"].as_bool().unwrap_or(false) {
        let error_message = auth_response["message"]
            .as_str()
            .unwrap_or("Invalid username/email or password")
            .to_string();
        return Err(error_message);
    }

    let tokens = auth_response["tokens"]
        .as_object()
        .ok_or("Invalid tokens in response")?;
    let access_token = tokens["access_token"]
        .as_str()
        .ok_or("Missing access_token")?
        .to_string();
    let refresh_token = tokens["refresh_token"]
        .as_str()
        .ok_or("Missing refresh_token")?
        .to_string();
    let user = auth_response["user"]
        .as_object()
        .ok_or("Invalid user in response")?;
    let user_id = user["id"].as_str().ok_or("Missing user id")?.to_string();
    let username = user["username"]
        .as_str()
        .ok_or("Missing username")?
        .to_string();

    Ok(AuthSuccess {
        access_token,
        refresh_token,
        user_id,
        username,
    })
}

#[tauri::command]
pub async fn refresh_token(
    refresh_token: String,
    user_id: String,
    app_handle: tauri::AppHandle,
) -> Result<AuthSuccess, String> {
    let payload = json!({
        "refresh_token": refresh_token,
        "user_id": user_id,
    });

    let auth_response = make_auth_request(&app_handle, "/auth/refresh", &payload, 200).await?;

    if !auth_response["success"].as_bool().unwrap_or(false) {
        let error_msg = auth_response["message"]
            .as_str()
            .unwrap_or("Failed to refresh token")
            .to_string();

        let _ = app_handle.emit("token_expired", ());

        return Err(error_msg);
    }

    let tokens = auth_response["tokens"]
        .as_object()
        .ok_or("Invalid tokens in response")?;
    let access_token = tokens["access_token"]
        .as_str()
        .ok_or("Missing access_token")?
        .to_string();
    let new_refresh_token = tokens["refresh_token"]
        .as_str()
        .ok_or("Missing refresh_token")?
        .to_string();

    Ok(AuthSuccess {
        access_token,
        refresh_token: new_refresh_token,
        user_id: String::new(),
        username: String::new(),
    })
}
