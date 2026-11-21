//! Command registry for the server CLI.
//!
//! Every command is defined once as a `CommandDef` row in a static table.
//! The row carries its help metadata (`summary`, `description`, `usage`,
//! `params`), its auth requirements, and its handler, so dispatch, help
//! output and auth gating are all derived from the same definition.
//!
//! Handlers are function pointers `fn(CmdCtx) -> BoxedFut` because the
//! commands table is a `static`; the `cmd!` macro wraps an existing async
//! function into that shape. All handler state arrives in an owned
//! [`CmdCtx`], so handlers carry no lifetimes.

use super::AdminSession;
use crate::db::DB;
use colored::Colorize;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::sync::watch;

/// A boxed future returned by a command handler.
pub type BoxedFut = Pin<Box<dyn Future<Output = ()> + Send>>;

/// A command handler as stored in the commands table.
pub type Handler = fn(CmdCtx) -> BoxedFut;

/// Everything a command handler can access.
pub struct CmdCtx {
    pub db: DB,
    pub raw: String,
    pub session: Arc<Mutex<AdminSession>>,
    pub server_start: Instant,
    pub shutdown_tx: watch::Sender<bool>,
    pub commands: &'static [&'static CommandDef],
}

/// Parameter metadata rendered by `help <command>`: (name, meaning).
pub type Param = (&'static str, &'static str);

/// A single server command definition.
pub struct CommandDef {
    pub name: &'static str,
    pub summary: &'static str,
    pub description: &'static str,
    pub usage: &'static str,
    pub requires_auth: bool,
    pub requires_admin: bool,
    pub params: &'static [Param],
    pub handler: Handler,
}

/// Shorthand for writing rows in the commands table.
///
/// The async handler is not a valid fn pointer until it is boxed, so the
/// macro wraps it in a closure that boxes its future.
#[macro_export]
macro_rules! cmd {
    (
        $name:literal, $summary:literal, $description:literal, $usage:literal,
        $requires_auth:expr, $requires_admin:expr, $params:expr, $handler:path
    ) => {
        $crate::cli::registry::CommandDef {
            name: $name,
            summary: $summary,
            description: $description,
            usage: $usage,
            requires_auth: $requires_auth,
            requires_admin: $requires_admin,
            params: $params,
            handler: |ctx: $crate::cli::registry::CmdCtx| -> $crate::cli::registry::BoxedFut {
                Box::pin(async move { $handler(ctx).await })
            },
        }
    };
}

/// Looks a command up by its full command name.
///
/// # Arguments
/// * `commands` - The active command table.
/// * `name` - The full command name to look up, e.g. `server:shutdown`.
///
/// # Return
/// Returns the matching `CommandDef`, or `None` if no command matches.
pub fn find<'c>(commands: &[&'c CommandDef], name: &str) -> Option<&'c CommandDef> {
    commands.iter().copied().find(|c| c.name == name)
}

/// Executes a parsed command against the active command table.
///
/// Admin-gated commands are rejected through `require_admin` before the
/// handler runs. Unrecognised commands print the `help` hint.
///
/// # Arguments
/// * `commands` - The active command table.
/// * `name` - The parsed command name, e.g. `server:shutdown`.
/// * `raw` - The raw remainder after the command name.
/// * `db` - The database connection.
/// * `session` - The current operator session.
/// * `server_start` - When the server started, used for uptime display.
/// * `shutdown_tx` - Shutdown signal channel used by `server:shutdown`.
pub async fn dispatch(
    commands: &'static [&'static CommandDef],
    name: &str,
    raw: &str,
    db: &DB,
    session: &Arc<Mutex<AdminSession>>,
    server_start: Instant,
    shutdown_tx: &watch::Sender<bool>,
) {
    let Some(def) = find(commands, name) else {
        unknown(raw);
        return;
    };

    if def.requires_admin && !super::require_admin(session) {
        return;
    }

    let ctx = CmdCtx {
        db: db.clone(),
        raw: raw.to_string(),
        session: session.clone(),
        server_start,
        shutdown_tx: shutdown_tx.clone(),
        commands,
    };

    (def.handler)(ctx).await;
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
