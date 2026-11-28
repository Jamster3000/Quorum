//! Configuration schema definition.
//!
//! This file is the single source of truth for the server's configuration,
//! in the same way `commands_list.rs` is the single source of truth for
//! commands. Every config field is declared once in the
//! `define_config_schema!` table below, and everything else (struct fields,
//! serialization, defaults, parsing, key lookup, `config show` output,
//! setup defaults) is generated from that one table.
//!
//! To add a new config field: add one row to the table. The field then
//! automatically exists on `ConfigFields`, is loaded/saved with the rest of
//! `secrets.enc`, accepts updates via `config <key> <value>`, and shows up
//! grouped under its section in `config show`.

#[macro_export]
macro_rules! define_config_schema {
    (
        $(
            $field_name:ident, $field_type:ty, $default_value:expr,
            $section:expr, $sensitive:expr, $description:expr
        ),* $(,)?
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

        /// Metadata about a single config field (name, section, default)
        pub struct ConfigFieldMeta {
            pub name: &'static str,
            /// Section the field is grouped under in `config show`
            pub section: &'static str,
            /// When true, the value is masked in `config show`
            pub sensitive: bool,
            pub default_str: &'static str,
            pub description: &'static str,
        }

        /// All config field metadata, in display order
        pub const CONFIG_SCHEMA: &[ConfigFieldMeta] = &[
            $(
                ConfigFieldMeta {
                    name: stringify!($field_name),
                    section: $section,
                    sensitive: $sensitive,
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
        // value lookup
        // ===================

        impl ConfigFields {
            /// Returns the value of a field by key, formatted as a string.
            ///
            /// # Arguments
            /// * `key` - The schema name of the field, e.g. `server_port`
            ///
            /// # Return
            /// Returns the field value as a string, or `None` if the key is
            /// not part of the schema.
            pub fn field_str(&self, key: &str) -> Option<String> {
                match key {
                    $(
                        stringify!($field_name) => Some(self.$field_name.to_string()),
                    )*
                    _ => None,
                }
            }
        }

        // ===================
        // sections
        // ===================

        /// Returns the config sections in schema order, without duplicates
        pub fn config_sections() -> Vec<&'static str> {
            let mut sections: Vec<&'static str> = Vec::new();
            for field in CONFIG_SCHEMA {
                if !sections.contains(&field.section) {
                    sections.push(field.section);
                }
            }
            sections
        }

        // ===================
        // field iterator
        // ===================

        /// Returns all field names as a slice
        pub const fn field_names() -> &'static [&'static str] {
            &[$(stringify!($field_name)),*]
        }
    };
}

#[rustfmt::skip]
define_config_schema!(
    // name, type, default, section, sensitive, description

    // Server
    server_port, u16, 3000, "Server", false, "HTTP port the server binds to (e.g. 3000)",
    server_host, String, "127.0.0.1".to_string(), "Server", false, "IP address the server binds to (e.g. 127.0.0.1 or 0.0.0.0)",
    server_url, String, "http://127.0.0.1:3000".to_string(), "Server", false, "Full server URL including protocol and port",

    // Database
    surreal_data_path, String, "./data/db".to_string(), "Database", false, "Path to the directory where SurrealDB stores its on-disk data",
    surreal_ns, String, "quorum".to_string(), "Database", false, "SurrealDB namespace to use",
    surreal_db, String, "quorum".to_string(), "Database", false, "SurrealDB database name within the namespace",

    // Authentication
    jwt_secret, String, "default-insecure-secret".to_string(), "Authentication", true, "Secret key used to sign and verify JWT tokens",
    jwt_access_minutes, i64, 15, "Authentication", false, "How long an access token remains valid, in minutes",
    jwt_refresh_days, i64, 7, "Authentication", false, "How long a refresh token remains valid, in days",

    // Testing & Rate Limits
    enable_testing, bool, false, "Testing & Rate Limits", false, "Whether to run the test suite on server startup",
    default_per_second, u64, 100, "Testing & Rate Limits", false, "Rate limit: sustained request rate for standard endpoints",
    default_burst_size, u32, 200, "Testing & Rate Limits", false, "Rate limit: maximum burst size for standard endpoints",
    testing_per_second, u64, 1000, "Testing & Rate Limits", false, "Rate limit: sustained request rate for test/dev endpoints",
    testing_burst_size, u32, 2000, "Testing & Rate Limits", false, "Rate limit: maximum burst size for test/dev endpoints"
);

pub type SerializableConfig = ConfigFields;
