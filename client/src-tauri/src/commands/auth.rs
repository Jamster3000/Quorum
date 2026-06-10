use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::common::make_auth_request;

#[derive(Serialize)]
pub struct SignupResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct SignupPayload {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
    pub confirm_password: String,
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