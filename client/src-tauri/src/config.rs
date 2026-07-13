use std::sync::OnceLock;

static SERVER_URL: OnceLock<String> = OnceLock::new();

pub fn init() {
	dotenvy::dotenv().ok();

    let url = std::env::var("SERVER_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());

    SERVER_URL.set(url).expect("Failed to set SERVER_URL");
}

/// Gets the url of the public SERVER_URL
///
pub fn get_server_url() -> &'static str {
    SERVER_URL.get_or_init(|| {
        dotenvy::dotenv().ok();
        std::env::var("SERVER_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string())
    })
}