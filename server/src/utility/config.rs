use std::sync::OnceLock;

pub struct Config {
    pub server_port: u16,
    pub server_url: String,
    pub server_host: String,
    pub surreal_url: String,
    pub surreal_user: String,
    pub surreal_pass: String,
    pub surreal_ns: String,
    pub surreal_db: String,
    pub jwt_secret: String,
    pub jwt_access_minutes: i64,
    pub jwt_refresh_days: i64,
    pub enable_testing: bool,
    pub default_per_second: u64,
    pub default_burst_size: u32,
    pub testing_per_second: u64,
    pub testing_burst_size: u32,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn load() -> Result<(), Box<dyn std::error::Error>> {
        let server_port: u16 = std::env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()?;

        let server_host = std::env::var("SERVER_HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());

        let config = Config {
            server_port,
            server_host: server_host.clone(),
            server_url: std::env::var("SERVER_URL")
                .unwrap_or_else(|_| format!("http://{}:{}", server_host, server_port)),
            surreal_url: std::env::var("SURREAL_URL")?,
            surreal_user: std::env::var("SURREAL_USER")?,
            surreal_pass: std::env::var("SURREAL_PASS")?,
            surreal_ns: std::env::var("SURREAL_NS")?,
            surreal_db: std::env::var("SURREAL_DB")?,
            jwt_secret: std::env::var("JWT_SECRET")?,
            jwt_access_minutes: std::env::var("JWT_ACCESS_MINUTES")?.parse()?,
            jwt_refresh_days: std::env::var("JWT_REFRESH_DAYS")?.parse()?,
            enable_testing: std::env::var("ENABLE_TESTS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()?,
            default_per_second: std::env::var("DEFAULT_PER_SECOND")
                .unwrap_or_else(|_| "2".to_string())
                .parse()?,
            default_burst_size: std::env::var("DEFAULT_BURST_SIZE")
                .unwrap_or_else(|_| "5".to_string())
                .parse()?,
            testing_per_second: std::env::var("TESTING_PER_SECOND")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            testing_burst_size: std::env::var("TESTING_BURST_SIZE")
                .unwrap_or_else(|_| "50".to_string())
                .parse()?,
        };

        CONFIG.set(config).map_err(|_| "Config already initialized".into())
    }

    pub fn get() -> &'static Config {
        CONFIG.get().expect("Config not initialized. Call Config::load() first.")
    }
}