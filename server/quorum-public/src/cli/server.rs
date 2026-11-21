use super::registry::CmdCtx;
use colored::Colorize;
use dialoguer::Input;
use quorum_core::utility::secrets::prompt_passphrase;
use quorum_core::utility::std::typewriter_println;
use zeroize::Zeroizing;

use crate::db::schema;
use quorum_core::db::{self as core_db, init::DB};
use quorum_core::startup;

/// Re-initializes the database schema.
///
/// Re-runs the schema definitions from initial.surql against the database.
///
/// # Errors
///
/// Returns an error when the schema initialization fails.
///
/// # Arguments
/// * `db` - A reference to the database connection.
pub async fn reinitialize_schema(db: &DB) -> Result<(), String> {
    let timer = startup::create_timer();

    // The schema error is converted to a string immediately: it arrives boxed as
    // `Box<dyn std::error::Error>`, which is not `Send`, so it must not be held
    // across any await inside this future (the command handlers below require
    // `Send` futures). Only the `String` message crosses the awaits.
    let result = schema::init(db).await.err().map(|error| error.to_string());

    match result {
        Some(message) => {
            startup::print_step("Re-initializing schema", false, startup::elapsed(timer));

            eprintln!("{}", format!("  Error: {}", message).red());

            let _ = core_db::queries::server_logs::log_error(db, message.clone(), 0).await;

            Err(message)
        }

        None => {
            startup::print_step("Re-initializing schema", true, startup::elapsed(timer));

            Ok(())
        }
    }
}

/// Handler for `server:update-database`.
///
/// # Arguments
/// * `ctx` - The command context, providing the database connection.
pub async fn update_database(ctx: CmdCtx) {
    if let Err(error) = reinitialize_schema(&ctx.db).await {
        eprintln!(
            "{}",
            format!("Failed to reinitialize schema: {error}").red()
        );
    }
}

/// Authenticates an admin session using the server passphrase.
///
/// Prompts for the passphrase and attempts to decrypt secrets.enc with it.
/// Success means the caller is whoever set up the server — session is granted.
/// The session expires after 20 minutes of inactivity as normal.
///
/// # Arguments
/// * `ctx` - The command context, providing the admin session.
///
/// # Example
/// ```rust,ignore
/// login(&session);
/// ```
pub async fn login(ctx: CmdCtx) {
    {
        let sess = ctx.session.lock().unwrap();
        if sess.is_valid() {
            println!("{}", "Already logged in. Run server:logout first.".yellow());
            return;
        }
    }

    let passphrase = match quorum_core::utility::secrets::prompt_passphrase() {
        Ok(p) => p,
        Err(e) => {
            println!("{}", format!("  Failed to read passphrase: {}", e).red());
            return;
        }
    };

    match quorum_core::utility::secrets::load_encrypted_config(&passphrase) {
        Ok(_) => {
            let mut sess = ctx.session.lock().unwrap();
            sess.login("admin".to_string(), true);
            println!("{}", "  Authenticated.".green().bold());
            println!(
                "{}",
                format!(
                    "  Session will expire after {} minutes of inactivity.",
                    quorum_core::cli::SESSION_TIMEOUT_MINS
                )
                .dimmed()
            );
        }
        Err(_) => {
            println!("{}", "  Invalid passphrase.".red());
        }
    }

    println!();
}

/// Prompts the user to confirm database deletion and initiates a reset.
///
/// Displays a confirmation prompt and requires the user to enter their server passphrase
/// to verify they have authority to perform this destructive operation. On confirmation,
/// deletes the entire database directory and recreates it empty, then gracefully shuts
/// down the server for restart.
///
/// The user will be prompted twice: once to confirm intent, and once to provide the passphrase.
/// Either prompt can be cancelled by answering "no" to the confirmation or entering an invalid passphrase.
///
/// # Arguments
/// * `shutdown_tx` - A watch channel sender used to signal server shutdown after a successful reset.
///
/// # Example
/// ```rust,ignore
/// confirm_and_delete(&shutdown_tx).await;
/// ```
pub async fn confirm_and_delete(shutdown_tx: &tokio::sync::watch::Sender<bool>) {
    println!();
    let _ = typewriter_println(&format!(
        "{}",
        "WARNING: This will delete all data and shut down the server.\nYou must restart it manually after.\n\nContinue? (y/n)".yellow()
    ))
        .map_err(|e| e.to_string());

    let mut confirm = Input::<String>::new()
        .with_prompt(" ")
        .interact_text()
        .unwrap();

    confirm = confirm.to_lowercase();

    if confirm != "y" && confirm != "yes" {
        return;
    }

    println!();
    let _ = typewriter_println(&format!(
        "{}",
        "Enter passphrase to confirm database deletion..."
            .cyan()
            .bold()
    ))
    .map_err(|e| e.to_string());

    match prompt_passphrase() {
        Ok(passphrase) => {
            let passphrase = Zeroizing::new(passphrase);
            if let Err(e) = perform_reset(&passphrase).await {
                eprintln!("{}", format!("Failed to reset database: {e}").red());
                return;
            }
            println!(
                "{}",
                "Database reset complete. The server is shutting down.\nPlease restart the server to continue.".green()
            );
            let _ = shutdown_tx.send(true);
        }
        Err(e) => {
            eprintln!("Failed to read passphrase: {}", e);
        }
    }
}

/// Handler for `db:delete`.
///
/// # Arguments
/// * `ctx` - The command context, providing the shutdown channel.
pub async fn db_delete(ctx: CmdCtx) {
    confirm_and_delete(&ctx.shutdown_tx).await;
}

/// Deletes and reinitializes the database with an empty schema.
///
/// Verifies the provided passphrase to ensure the caller has authority, then spawns a blocking
/// task to delete the entire database directory and recreate it. After the filesystem operations
/// complete, reinitializes the database schema from the initial migration script.
///
/// The database deletion is performed on a separate thread to avoid holding locks from the CLI's
/// database reference, which would prevent the filesystem operations from succeeding on Windows.
///
/// # Errors
///
/// Returns an error when:
/// - the passphrase is invalid,
/// - the database directory cannot be deleted or recreated,
/// - or schema initialization fails.
///
/// # Arguments
/// * `passphrase` - The server passphrase, used to verify the caller's authority before performing the reset.
async fn perform_reset(passphrase: &str) -> Result<(), Box<dyn std::error::Error>> {
    quorum_core::utility::config::Config::load_with_passphrase(passphrase)?;

    let config = quorum_core::utility::config::Config::get();
    let path = std::path::Path::new(&config.surreal_data_path).to_path_buf();

    tokio::task::spawn_blocking(move || {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        if path.exists() {
            std::fs::remove_dir_all(&path).ok();
        }
        std::fs::create_dir_all(&path).ok();
    })
    .await?;

    Ok(())
}
