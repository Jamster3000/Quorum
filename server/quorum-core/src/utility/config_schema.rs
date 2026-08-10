//! Configuration schema definition.
//!
//! This file defines the configuration schema in a declarative way using macros.
//! Every config field is defined here once, and all related code (struct fields, serialization,
//! parsing, display, defaults, etc.) is generated from this single definition.

#[macro_export]
macro_rules! define_config_schema {
    (
        $($field_name:ident, $field_type:ty, $default_value:expr, $description:expr),* $(,)?
    ) => {
        // ===================
        // structs and types
        // ===================

        /// config settings loaded from encrypted secrets.enc
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #[serde(default)]
        pub struct ConfigFields {
            $(
                #[doc = $description]
                pub $field_name: $field_type,
            )*
        }

        /// Serializable config for disk storage (encryption/decryption)
        pub type SerializableConfigFields = ConfigFields;

        // ===================
        // metadata
        // ===================

        /// Metadata about a single config field (name, type, default)
        pub struct ConfigFieldMeta {
            pub name: &'static str,
            pub default_str: &'static str,
            pub description: &'static str,
        }

        /// All config field metadata
        pub const CONFIG_SCHEMA: &[ConfigFieldMeta] = &[
            $(
                ConfigFieldMeta {
                    name: stringify!($field_name),
                    default_str: stringify!($default_value),
                    description: $description,
                },
            )*
        ];

        // ===================
        // defaults
        // ===================

        /// Returns a new ConfigFields with all default values
        pub fn default_config_fields() -> ConfigFields {
            ConfigFields {
                $(
                    $field_name: $default_value,
                )*
            }
        }

        impl Default for ConfigFields {
            fn default() -> Self {
                default_config_fields()
            }
        }

        // ===================
        // parser
        // ===================

        /// Parses a config key-value pair and updates the fields struct
        /// Returns Ok(true) if the key was recognized, Ok(false) if unknown, Err on parse failure
        pub fn parse_config_field(
            fields: &mut ConfigFields,
            key: &str,
            value: &str,
        ) -> Result<bool, String> {
            match key {
                $(
                    stringify!($field_name) => {
                        fields.$field_name = value.parse()
                            .map_err(|_| format!("Failed to parse {} as {}", key, stringify!($field_type)))?;
                        Ok(true)
                    }
                )*
                _ => Ok(false),
            }
        }

        // ===================
        // Field iterator
        // ===================

        /// Returns all field names as a slice
        pub const fn field_names() -> &'static [&'static str] {
            &[$(stringify!($field_name)),*]
        }
    };
}

define_config_schema!(
    // Server
    server_port,
    u16,
    3000,
    "HTTP port the server binds to (e.g. 3000)",
    server_host,
    String,
    "127.0.0.1".to_string(),
    "IP address the server binds to (e.g. 127.0.0.1 or 0.0.0.0)",
    server_url,
    String,
    "http://127.0.0.1:3000".to_string(),
    "Full server URL including protocol and port",
    // Database
    surreal_data_path,
    String,
    "./data/db".to_string(),
    "Path to the directory where SurrealDB stores its on-disk data",
    surreal_ns,
    String,
    "quorum".to_string(),
    "SurrealDB namespace to use",
    surreal_db,
    String,
    "quorum".to_string(),
    "SurrealDB database name within the namespace",
    // JWT
    jwt_secret,
    String,
    "default-insecure-secret".to_string(),
    "Secret key used to sign and verify JWT tokens",
    jwt_access_minutes,
    i64,
    15,
    "How long an access token remains valid, in minutes",
    jwt_refresh_days,
    i64,
    7,
    "How long a refresh token remains valid, in days",
    // testing features
    enable_testing,
    bool,
    false,
    "Whether to run the test suite on server startup",
    // Rate Limiting
    default_per_second,
    u64,
    100,
    "Rate limit: sustained request rate for standard endpoints",
    default_burst_size,
    u32,
    200,
    "Rate limit: maximum burst size for standard endpoints",
    testing_per_second,
    u64,
    1000,
    "Rate limit: sustained request rate for test/dev endpoints",
    testing_burst_size,
    u32,
    2000,
    "Rate limit: maximum burst size for test/dev endpoints"
);

pub type SerializableConfig = ConfigFields;
