//! This is the main entry point for the Quorum Public server.
//!
//! Use `cargo run -p quorum-public` to build/run the public version,
//! which combines the `quorum-core` shared code.

mod cli;
mod db;
mod models;
mod routes;
mod tests;
mod utility;

use cli::PUBLIC_COMMANDS;
use quorum_core::server::{ServerOptions, run_server};
use routes::route::create_router;
use std::future::Future;
use std::pin::Pin;

#[tokio::main]
async fn main() {
    run_server(ServerOptions {
        schema: include_str!("../initial.surql"),
        router: create_router,
        extra_commands: PUBLIC_COMMANDS,
        run_tests: Some(|| {
            Box::pin(tests::run_all_tests()) as Pin<Box<dyn Future<Output = ()> + Send>>
        }),
        block_ctrl_c: cfg!(not(debug_assertions)),
    })
    .await;
}
