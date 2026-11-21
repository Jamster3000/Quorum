//! Main entry point for the server CLI.
//!
//! Owns the admin session, input parsing and the blocking input loop.
//! Commands themselves are defined in `registry.rs` and implemented in
//! `server.rs`, `db.rs` and `help.rs`.

pub mod db;
pub mod help;
pub mod registry;
pub mod server;
//TODO: Most of this code needs to be moved into a new file not mod.rs
use crate::cli::registry::CommandDef;
use crate::db::init::DB;
use colored::Colorize;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::watch;

pub const SESSION_TIMEOUT_MINS: u64 = 20;

pub struct AdminSession {
    pub logged_in: bool,
    pub last_active: Instant,
    pub username: Option<String>,
    pub is_admin: bool,
}

impl Default for AdminSession {
    fn default() -> Self {
        Self {
            logged_in: false,
            is_admin: false,
            last_active: Instant::now(),
            username: None,
        }
    }
}

/// Represents the state of an admin session in the CLI.
impl AdminSession {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_valid(&self) -> bool {
        self.logged_in
            && self.last_active.elapsed() < Duration::from_secs(SESSION_TIMEOUT_MINS * 60)
    }

    pub fn update_is_admin(&mut self, is_admin: bool) {
        self.is_admin = is_admin;
    }

    pub fn login(&mut self, username: String, is_admin: bool) {
        self.logged_in = true;
        self.username = Some(username);
        self.is_admin = is_admin;
        self.last_active = Instant::now();
    }

    pub fn logout(&mut self) {
        self.logged_in = false;
        self.username = None;
    }
}

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

/// Parses an input line into the command name and its raw arguments.
///
/// Commands use a colon between the command category and the actual command,
/// making parsing very simple and easy to do.
///
/// # Arguments
/// * `input` - A string slice that holds the command input.
///
/// # Return
/// Returns `Some((name, raw_args))` such as `("db:table", "users, 2")`, or
/// `None` if the input is empty.
fn parse(input: &str) -> Option<(String, &str)> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    let (name_part, raw) = trimmed.split_once(' ').unwrap_or((trimmed, ""));

    let name = name_part
        .split(':')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(":");

    if name.is_empty() {
        return None;
    }

    Some((name, raw.trim()))
}

/// Spawns the CLI in a separate blocking task.
///
/// The server sits on the main thread/task, whilst the CLI runs on a separate
/// task to avoid holding the server from handling requests. Commands are
/// resolved against `registry::CORE_COMMANDS` combined with `extra_commands`,
/// which is how each server binary registers its own commands on top of the
/// shared ones.
///
/// # Arguments
/// * `extra_commands` - Commands specific to this server binary, on top of the core commands.
/// * `db` - A reference to the database connection.
/// * `server_start` - The instant when the server started, used for uptime calculations.
/// * `shutdown_tx` - A watch channel sender to signal server shutdown.
pub async fn spawn_cli(
    extra_commands: &'static [CommandDef],
    db: DB,
    server_start: Instant,
    shutdown_tx: watch::Sender<bool>,
) {
    let session = Arc::new(Mutex::new(AdminSession::new()));
    let handle = tokio::runtime::Handle::current();

    let commands: Vec<&'static CommandDef> =
        CORE_COMMANDS.iter().chain(extra_commands.iter()).collect();

    let commands: &'static [&'static CommandDef] = Box::leak(commands.into_boxed_slice());

    tokio::task::spawn_blocking(move || {
        std::thread::sleep(Duration::from_millis(600));

        loop {
            print!("{} ", ">".cyan().bold());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => break, // EOF
                Ok(_) => {}
                Err(_) => break,
            }

            {
                let mut sess = session.lock().unwrap();
                if sess.logged_in && !sess.is_valid() {
                    println!(
                        "{}",
                        "Session expired after 20 minutes of inactivity. Please run server:login again."
                            .yellow()
                    );
                    sess.logout();
                }
            }

            let Some((name, raw)) = parse(&input) else {
                continue;
            };

            handle.block_on(registry::dispatch(
                commands,
                &name,
                raw,
                &db,
                &session,
                server_start,
                &shutdown_tx,
            ));
        }
    });
}

/// Checks if a command requires admin privileges.
///
/// Uses the `AdminSession` built struct to determine if the current session is valid and has admin privileges.
/// If not, it prints an error message and returns false, stopping the command from executing.
///
/// The intention of this functionality is so that if someone was to have gotten unauthorised access to the actively running server
/// and try to run a command, this safeguards against them running anything dangerous or destructive without first authenticating as an admin user.
/// Which without the correct account information of the admin user, they are unable to do any serious harm.
///
/// # Arguments
/// * `session` - A reference to the admin session, wrapped in Arc<Mutex<>> for thread-safe access.
///
/// # Return
/// * `bool` - Returns true if the session is valid and has admin privileges, false otherwise.
///
/// # Example
/// ```rust
/// if !require_admin(session) {
///     return;
/// }
/// ```
pub fn require_admin(session: &Arc<Mutex<AdminSession>>) -> bool {
    let sess = session.lock().unwrap();
    if !sess.is_valid() {
        println!(
            "{}",
            "This command requires authentication. Please run server:login first.".red()
        );
        return false;
    }
    if !sess.is_admin {
        println!("{}", "This command requires admin privileges.".red());
        return false;
    }
    true
}
