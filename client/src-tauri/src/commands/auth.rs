use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::common::make_auth_request;

#[derive(Serialize)]
pub struct SignupResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub user_id: Option<String>,
    pub username: Option<String>,
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
pub async fn signup(payload: SignupPayload) -> Result<SignupResponse, String> {
    if payload.username.len() < 1 {
        return Ok(SignupResponse {
            success: false,
            message: "Username must be at least 1 characters.".to_string(),
        });
    }

    if payload.username.len() > 18 {
        return Ok(SignupResponse {
            success: false,
            message: "Username must be under 32 characters.".to_string(),
        });
    }

    if let Some(ref email) = payload.email {
        if !email.is_empty() && !email.contains('@') {
            return Ok(SignupResponse {
                success: false,
                message: "Please enter a valid email address.".to_string(),
            });
        }
    }

    if payload.password.is_empty() {
        return Ok(SignupResponse {
            success: false,
            message: "Password cannot be empty.".to_string(),
        });
    }

    if payload.password.len() < 8 {
        return Ok(SignupResponse {
            success: false,
            message: "Password must be at least 8 characters.".to_string(),
        });
    }

    if payload.password.len() > 128 {
        return Ok(SignupResponse {
            success: false,
            message: "Password must be under 128 characters.".to_string(),
        });
    }

    if payload.password != payload.confirm_password {
        return Ok(SignupResponse {
            success: false,
            message: "Passwords do not match.".to_string(),
        });
    }

    let mut new_payload = json!({
        "username": payload.username,
        "email": payload.email,
        "password": payload.password,
    });

    if let Some(ref email) = payload.email {
        if email.is_empty() {
            new_payload = json!({
                "username": payload.username,
                "password": payload.password,
            });
        }
    }


    make_auth_request("/auth/signup", &new_payload, 201).await?;

    Ok(SignupResponse {
        success: true,
        message: "Account created successfully.".to_string(),
    })
}

#[tauri::command]
pub async fn login(payload: LoginPayload) -> Result<LoginResponse, String> {
    let payload = json!({
        "username_or_email": payload.username_or_email,
        "password": payload.password,
    });

    let auth_response = make_auth_request("/auth/login", &payload, 200).await?;

    println!("Server response: {:?}", auth_response);

    //Extract the tokens the user needs from the server response
    let tokens = auth_response["tokens"].as_object().ok_or("Invalid tokens in response")?;
    let access_token = tokens["access_token"].as_str().map(|s| s.to_string());
    let refresh_token = tokens["refresh_token"].as_str().map(|s| s.to_string());

    let user = auth_response["user"].as_object().ok_or("Invalid user in response")?;
    let user_id = user["id"].as_str().map(|s| s.to_string());
    let username = user["username"].as_str().map(|s| s.to_string());

    Ok(LoginResponse {
        success: true,
        message: auth_response["message"].as_str().unwrap_or("Logged in successfully.").to_string(),
        access_token,
        refresh_token,
        user_id,
        username,
    })
}