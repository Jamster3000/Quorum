//! Main entry point for the server cli Commands.
//!
//! Commands shared by every Quorum server are defined in `quorum_core`'s
//! command registry; the table below registers the commands that only the
//! public server has.

pub mod config;
pub mod server;
pub mod test;
pub mod user;

use quorum_core::cli::registry::{self, CommandDef};

//TODO: need to move stuff from here to a new file

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
pub static PUBLIC_COMMANDS: &[CommandDef] = &[
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
        "server:update-database",
        "Re-initializes the database schema.",
        "Re-runs the schema definitions from the initial migration against the database, adding any tables or fields that are missing without touching existing data.",
        "server:update-database",
        false,
        false,
        &[],
        server::update_database
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
    quorum_core::cmd!(
        "user:delete",
        "Deletes a user account.",
        "Removes a user account from the database. This action is irreversible.",
        "user:delete <id>",
        true,
        true,
        &[("id", "The ID of the user to delete")],
        user::delete
    ),
    quorum_core::cmd!(
        "test:run",
        "Runs functional tests on the server.",
        "Executes a suite of functional tests to verify that the server is operating correctly. This is only available if testing mode is enabled.",
        "test:run",
        false,
        false,
        &[],
        test::run
    ),
];
