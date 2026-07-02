//! Main entry point for the server cli Commands
//! This is where commands are defined properly, and where command functions are called.

pub mod help;
pub mod server;
pub mod test;
pub mod user;

use crate::cli::server::confirm_and_delete;
use colored::Colorize;
use quorum_core::cli::AdminSession;
use quorum_core::cli::db;
use quorum_core::cli::server::{audit, logout, logs, shutdown, status};
use quorum_core::db::DB;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::watch;

const SESSION_TIMEOUT_MINS: u64 = 20;

struct Command {
    parts: Vec<String>,
    #[allow(dead_code)]
    params: Vec<String>,
    raw: String,
}

/// Parses a command string into a Command struct.
///
/// Commands use colon between the command category and the actual command,
/// making parsing very simple and easy to do.
///
/// # Arguments
/// * `input` - A string slice that holds the command input.
///
/// # Returns
/// * `Option<Command>` - Returns Some(Command) if parsing is successful, or None if the input is empty.
fn parse(input: &str) -> Option<Command> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    let (cmd_part, params_part) = match trimmed.split_once(' ') {
        Some((cmd, params)) => (cmd, params.trim()),
        None => (trimmed, ""),
    };

    let parts: Vec<String> = cmd_part
        .split(':')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    let params: Vec<String> = if params_part.is_empty() {
        vec![]
    } else {
        params_part
            .split(", ")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    };

    Some(Command {
        parts,
        params,
        raw: trimmed.to_string(),
    })
}

/// Spawns the CLI in a separate blocking thread.
///
/// Runs an interactive command loop that listens for user input and dispatches commands
/// to their respective handlers. The CLI operates on a dedicated thread to avoid blocking
/// the async Tokio runtime, allowing the server to continue handling HTTP requests
/// concurrently.
///
/// Session expiry is enforced—after 20 minutes of inactivity, users must re-authenticate
/// with `server:login` before executing protected commands.
///
/// The CLI automatically exits when a shutdown signal is received via `shutdown_tx`.
///
/// # Arguments
/// * `db` - A reference to the database connection, passed to all command handlers.
/// * `server_start` - The instant when the server started, used for uptime calculations in `server:status`.
/// * `shutdown_tx` - A watch channel sender that signals the CLI to exit when the server is shutting down.
///
/// # Example
/// ```rust
/// let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
/// cli::spawn_cli(db, server_start, shutdown_tx).await;
/// ```
pub async fn spawn_cli(db: DB, server_start: Instant, shutdown_tx: watch::Sender<bool>) {
    let session = Arc::new(Mutex::new(AdminSession::new()));
    let handle = tokio::runtime::Handle::current();
    let shutdown_rx = shutdown_tx.subscribe();

    tokio::task::spawn_blocking(move || {
        std::thread::sleep(Duration::from_millis(600));

        loop {
            // Check if shutdown was signaled
            if shutdown_rx.has_changed().unwrap_or(false) {
                break;
            }

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

            let cmd = match parse(&input) {
                Some(c) => c,
                None => continue,
            };

            handle.block_on(dispatch(&cmd, &db, server_start, &session, &shutdown_tx));
        }
    });
}

/// Dispatches the parsed command to the appropriate handler function.
///
/// # Arguments
/// * `cmd` - A reference to the parsed Command struct.
/// * `db` - A reference to the database connection.
/// * `server_start` - The instant when the server started, used for uptime calculations.
/// * `session` - A reference to the admin session, wrapped in Arc<Mutex<>> for thread-safe access.
/// * `shutdown_tx` - A watch channel sender to signal server shutdown.
async fn dispatch(
    cmd: &Command,
    db: &DB,
    server_start: Instant,
    session: &Arc<Mutex<AdminSession>>,
    shutdown_tx: &watch::Sender<bool>,
) {
    match cmd.parts.as_slice() {
        // -- help --
        [ns] if ns == "help" => {
            help::print_all();
        }
        [ns, command] if ns == "help" => {
            help::print_command(command);
        }

        // -- Server --
        [ns, command] if ns == "server" => match command.as_str() {
            "signup" => {
                server::signup(db).await;
            }
            "login" => server::login(db, session).await,
            "make-admin" => {
                let username = cmd.raw.split_once(' ').map(|x| x.1).unwrap_or("");
                server::make_admin(db, username, session).await;
            }
            "status" => status(server_start).await,
            "logout" => logout(session),
            "shutdown" => {
                if !require_admin(session) {
                    return;
                }
                shutdown(db, server_start, shutdown_tx).await;
            }
            "logs" => {
                let params = cmd.raw.split_once(' ').map(|x| x.1).unwrap_or("");
                logs(db, params).await;
            }
            "audit" => {
                let params = cmd.raw.split_once(' ').map(|x| x.1).unwrap_or("");
                audit(db, params).await;
            }
            _ => unknown(&cmd.raw),
        },

        // -- User --
        [ns, command] if ns == "user" => match command.as_str() {
            "delete" => {
                if !require_admin(session) {
                    return;
                }

                let id = cmd.raw.split_once(' ').map(|x| x.1).unwrap_or("");
                user::delete(db, id).await;
            }
            _ => unknown(&cmd.raw),
        },

        // -- Test --
        [ns, command] if ns == "test" => match command.as_str() {
            "run" => test::run().await,
            _ => unknown(&cmd.raw),
        },

        // -- Database --
        [ns, command] if ns == "db" => match command.as_str() {
            "stats" => db::stats(db).await,
            "table" => {
                let params = cmd.raw.split_once(' ').map(|x| x.1).unwrap_or("");
                db::table(db, params).await;
            }
            "delete" => {
                /*if !require_admin(session) {
                    return;
                }*/
                confirm_and_delete(shutdown_tx).await;
            }
            _ => unknown(&cmd.raw),
        },

        _ => unknown(&cmd.raw),
    }
}

/// Checks if a command required admin privileges.
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
/// # Returns
/// * `bool` - Returns true if the session is valid and has admin privileges, false otherwise.
///
/// # Example
/// ```rust
/// if !require_admin(session) {
///     return;
/// }
///```
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

/// Prints an error message for unknown commands.
///
/// # Arguments
/// * `raw` - The raw command string that was not recognized.
fn unknown(raw: &str) {
    println!(
        "{} {} {}",
        "Unknown command:".red(),
        raw.white(),
        "- type `help` to see available commands.".dimmed()
    );
}
