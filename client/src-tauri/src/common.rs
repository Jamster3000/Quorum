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
    let server_url = "http://127.0.0.1:3000";

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
        return Err(error_msg.to_string());
    }

    if body["success"] != true {
        return Err(format!("Request failed: {}", body["message"]));
    }

    Ok(body)
}
