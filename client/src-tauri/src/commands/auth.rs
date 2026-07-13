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
    println!("testone");
    if payload.username.is_empty() {
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

    println!("Signup payload: {:?}", new_payload);

    make_auth_request("/auth/signup", &new_payload, 201).await?;

    Ok(SignupResponse {
        success: true,
        message: "Account created successfully.".to_string(),
    })
}

#[tauri::command]
pub async fn login(payload: LoginPayload) -> Result<LoginResponse, String> {
    let request_payload = json!({
        "username_or_email": payload.username_or_email,
        "password": payload.password,
    });

    // Capture the result of make_auth_request (don't use ? yet)
    let auth_response = match make_auth_request("/auth/login", &request_payload, 200).await {
        Ok(response) => {
            response
        }
        Err(e) => {
            return Ok(LoginResponse {
                success: false,
                message: e,
                access_token: None,
                refresh_token: None,
                user_id: None,
                username: None,
            });
        }
    };

    println!("Auth response: {:?}", auth_response);

    if !auth_response["success"].as_bool().unwrap_or(false) {
        let error_message = auth_response["message"].as_str().unwrap_or("Invalid username/email or password").to_string();
        return Ok(LoginResponse {
            success: false,
            message: error_message,
            access_token: None,
            refresh_token: None,
            user_id: None,
            username: None,
        });
    }

    // Extract tokens and user data...
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

#[tauri::command]
pub async fn refresh_token(refresh_token: String) -> Result<LoginResponse, String> {
    let payload = json!({
        "refresh_token": refresh_token,
    });

    let auth_response = make_auth_request("/auth/refresh", &payload, 200).await?;

    if !auth_response["success"].as_bool().unwrap_or(false) {
        let error_msg = auth_response["message"].as_str().unwrap_or("Failed to refresh token").to_string();
        return Ok(LoginResponse {
            success: false,
            message: error_msg,
            access_token: None,
            refresh_token: None,
            user_id: None,
            username: None,
        });
    }

    let tokens = auth_response["tokens"].as_object().ok_or("Invalid tokens in response")?;
    let access_token = tokens["access_token"].as_str().map(|s| s.to_string());
    let new_refresh_token = tokens["refresh_token"].as_str().map(|s| s.to_string());

    Ok(LoginResponse {
        success: true,
        message: auth_response["message"].as_str().unwrap_or("Token refreshed successfully.").to_string(),
        access_token,
        refresh_token: new_refresh_token,
        user_id: None,
        username: None,
    })
}