use serde_json::json;
use reqwest;

const SERVER_URL: &str = "http://127.0.0.1:3000";

// Helper function to make HTTP requests
async fn make_request<T: serde::ser::Serialize>(
    method: &str,
    endpoint: &str,
    body: Option<T>,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = format!("{}{}", SERVER_URL, endpoint);

    let response = match method {
        "GET" => client.get(&url).send().await,
        "POST" => {
            let mut req = client.post(&url);
            if let Some(b) = body {
                req = req.json(&b);
            }
            req.send().await
        }
        _ => return Err("Unsupported method".to_string()),
    };

    let response = response.map_err(|e| e.to_string())?;
    response.text().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search(query: String) -> Result<String, String> {
    make_request("POST", "/search", Some(json!({ "query": query }))).await
}