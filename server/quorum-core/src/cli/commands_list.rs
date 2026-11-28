//! Main entry point for the server CLI.
//!
//! Owns the admin session, input parsing and the blocking input loop.
//! Commands themselves are defined in `registry.rs` and implemented in
//! `server.rs`, `db.rs` and `help.rs`.

use super::commands::{config, db, help, server};
use crate::cli::registry::CommandDef;

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
pub static CORE_COMMANDS: &[CommandDef] = &[
    crate::cmd!(
        "help",
        "Lists all available commands for the server.",
        "With no arguments, prints a summary of every available command, grouped by namespace. \
         With a command name, prints the full description, usage, parameters, and auth requirement.",
        "help [command]",
        false,
        false,
        &[("command", "Optional command to look up, e.g. server:status")],
        help::help
    ),
    crate::cmd!(
        "config:reload",
        "Reloads the configuration from secrets.enc without a restart.",
        "Prompts for the server passphrase, re-reads secrets.enc, and applies the loaded values immediately. Changes made through `config <key> <value>` are already live, so this is mainly for edits made to the file outside the running server.",
        "config:reload",
        false,
        true,
        &[],
        config::reload
    ),
    crate::cmd!(
        "config:reset",
        "Resets a configuration key to its default value.",
        "Looks up the key's default in the config schema, applies it, and persists it to secrets.enc. Run `config keys` to see all valid keys and their defaults.",
        "config:reset <key>",
        false,
        true,
        &[("key", "The config key to reset, e.g. server_port")],
        config::reset
    ),
    crate::cmd!(
        "server:version",
        "Shows the server version.",
        "Prints the Quorum server version, taken from the crate metadata at build time.",
        "server:version",
        false,
        false,
        &[],
        server::version
    ),
    crate::cmd!(
        "server:status",
        "Shows uptime and basic server info.",
        "Prints how long the server has been running, what address it is listening on, and whether testing mode is active.",
        "server:status",
        false,
        false,
        &[],
        server::status
    ),
    crate::cmd!(
        "server:logout",
        "Ends the current operator session.",
        "Clears the authenticated session immediately. Any subsequent write commands will require logging in again.",
        "server:logout",
        true,
        false,
        &[],
        server::logout
    ),
    crate::cmd!(
        "server:shutdown",
        "Gracefully shuts the server down.",
        "Signals the server to stop accepting new requests, waits for in-flight requests to complete, flushes logs, and exits cleanly.",
        "server:shutdown",
        true,
        true,
        &[],
        server::shutdown
    ),
    crate::cmd!(
        "server:logs",
        "Displays recent server logs.",
        "Prints logs in order from most recent to oldest, allowing for optional filter for last X days.",
        "server:logs [days]",
        false,
        false,
        &[("days", "Optional number of days to display logs for")],
        server::logs
    ),
    crate::cmd!(
        "server:audit",
        "Displays recent audit logs.",
        "Prints audit logs in order from most recent to oldest, allowing for optional filter for last X days.",
        "server:audit [days]",
        false,
        false,
        &[("days", "Optional number of days to display audit logs for")],
        server::audit
    ),
    crate::cmd!(
        "db:stats",
        "Displays database statistics.",
        "Prints the number of rows in each table, total row count, and estimated size of each table and database.",
        "db:stats",
        false,
        false,
        &[],
        db::stats
    ),
    crate::cmd!(
        "db:table",
        "Displays records from a specific table.",
        "Prints the records from the specified table, with an option to view a specific page of results.",
        "db:table <name>, <page>",
        false,
        false,
        &[
            ("name", "The name of the table to display"),
            ("page", "The page of results to display")
        ],
        db::table
    ),
];
