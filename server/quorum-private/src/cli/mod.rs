//! Main entry point for the server cli Commands.
//!
//! Commands shared by every Quorum server are defined in `quorum_core`'s
//! command registry; the table below registers the commands that only the
//! private (self-hosted) server has.

pub mod config;
pub mod server;

use quorum_core::cli::registry::{self, CommandDef};
use quorum_core::db::DB;
use std::time::Instant;
use tokio::sync::watch;

/// Source of truth for commands
///
/// The commands are formatted as follows
/// `name` - name of the command
/// `Summary` - Short summary of what the command is
/// `description` - the full length of the description
/// `usage` - How to use the command
/// `requires_auth` - true if it requires to be authenticated or not
/// `requires_admin` - true if it requires user to be the administrator
/// `params` - parameters that the command takes along with the meaning/description of that parameter
/// `handler` - the function that it should run when the command is entered
pub static PRIVATE_COMMANDS: &[CommandDef] = &[
    quorum_core::cmd!(
        "config:config",
        "Views or updates the server configuration.",
        "With `show`, prints the full configuration. With a key and a value, updates that setting and persists it to secrets.enc.",
        "config [show | <key> <value>]",
        false,
        true,
        &[
            ("show", "Prints the full configuration"),
            ("key", "The config key to update, e.g. server_host"),
            ("value", "The new value for the key"),
        ],
        config::config
    ),
    quorum_core::cmd!(
        "server:login",
        "Authenticates an operator session in the terminal.",
        "Prompts for the server passphrase and verifies it against secrets.enc. On success, grants access to write/destructive commands for the next 20 minutes of inactivity.",
        "server:login",
        false,
        false,
        &[],
        server::login
    ),
    quorum_core::cmd!(
        "db:delete",
        "Deletes ALL database data after confirmation.",
        "Prompts for confirmation, then requires the server passphrase to verify authority. Deletes the entire database directory and recreates it empty, then shuts the server down for a manual restart.",
        "db:delete",
        true,
        true,
        &[],
        server::db_delete
    ),
];

/// Spawns the CLI with the core commands plus `PRIVATE_COMMANDS`.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `server_start` - The instant when the server started, used for uptime calculations.
/// * `shutdown_tx` - A watch channel sender to signal server shutdown.
///
/// # Example
/// ```rust,ignore
/// cli::spawn_cli(db, server_start, shutdown_tx).await;
/// ```
pub async fn spawn_cli(db: DB, server_start: Instant, shutdown_tx: watch::Sender<bool>) {
    quorum_core::cli::spawn_cli(PRIVATE_COMMANDS, db, server_start, shutdown_tx).await;
}
