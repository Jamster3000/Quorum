use super::registry::CmdCtx;
use colored::Colorize;
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

    let result = quorum_core::server::init_schema(db, include_str!("../../initial.surql"))
        .await
        .err()
        .map(|error| error.to_string());

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
