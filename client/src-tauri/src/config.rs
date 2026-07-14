use std::sync::OnceLock;

static SERVER_URL: OnceLock<String> = OnceLock::new();

pub fn init() {
    let url = "http://127.0.0.1:3000".to_string();

    SERVER_URL.set(url).expect("Failed to set SERVER_URL");
}

/// Gets the url of the public SERVER_URL
///
/// Currently hardcoded -> Will need updating at later date
/// # Returns
/// A static string slice representing the server URL.
pub fn get_server_url() -> &'static str {
    "http://127.0.0.1:3000"
}
