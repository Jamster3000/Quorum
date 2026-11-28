//! Main entry point for the server cli Commands.
//!
//! Commands shared by every Quorum server are defined in `quorum_core`'s
//! command registry; the table below registers the commands that only the
//! public server has.

use super::commands::{server, test, user};
use quorum_core::cli::commands::server::{db_delete, login};
use quorum_core::cli::registry::CommandDef;

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
        "With no arguments or `show`, prints the full configuration. With `keys`, lists every configuration key with its description and default. With a key, prints that key's current value and details. With a key and a value, updates that setting and persists it to secrets.enc.",
        "config [show | keys | <key> [<value>]]",
        false,
        true,
        &[
            ("show", "Prints the full configuration"),
            (
                "keys",
                "Lists all configuration keys with descriptions and defaults"
            ),
            ("key", "The config key to view or update, e.g. server_host"),
            (
                "value",
                "The new value for the key; omit it to just view the key"
            ),
        ],
        quorum_core::cli::commands::config::config
    ),
    quorum_core::cmd!(
        "server:login",
        "Authenticates an operator session in the terminal.",
        "Prompts for the server passphrase and verifies it against secrets.enc. On success, grants access to write/destructive commands for the next 20 minutes of inactivity.",
        "server:login",
        false,
        false,
        &[],
        login
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
        db_delete
    ),
    quorum_core::cmd!(
        "user:list",
        "Lists user accounts.",
        "Prints all user accounts with their ID, username, email, creation date, and status. An optional search term filters by ID, username, or email, matched as a case-insensitive substring.",
        "user:list [search]",
        true,
        true,
        &[(
            "search",
            "Optional term matched against ID, username, or email"
        )],
        user::list
    ),
    quorum_core::cmd!(
        "user:show",
        "Shows the full details of one user account.",
        "Resolves the search term against IDs, usernames, and emails, then prints every non-sensitive field of the matching user. If several users match, they are listed so you can narrow the search. Sensitive fields (password hash, 2FA secret, backup codes) are never displayed.",
        "user:show <id | username | email>",
        true,
        true,
        &[("search", "The ID, username, or email of the user to show")],
        user::show
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
