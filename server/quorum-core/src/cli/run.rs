use crate::cli::commands_list::CORE_COMMANDS;
use crate::cli::registry::{CommandDef, dispatch};
use crate::db::init::DB;
use crate::models::admin::AdminSession;
use colored::Colorize;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::watch;

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
                Ok(0) => break,
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

            handle.block_on(dispatch(
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
