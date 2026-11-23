//! This is the main entry point for the Quorum Private server.
//!
//! Use `cargo run -p quorum-private` to build/run the private version,
//! which combines the `quorum-core` shared code.

mod cli;
mod routes;

use cli::PRIVATE_COMMANDS;
use quorum_core::server::{ServerOptions, run_server};
use routes::route::create_router;

#[tokio::main]
async fn main() {
    run_server(ServerOptions {
        schema: include_str!("../initial.surql"),
        router: create_router,
        extra_commands: PRIVATE_COMMANDS,
        run_tests: None,
        block_ctrl_c: false,
    })
    .await;
}
