mod db;
mod routes;
mod startup;
mod utility;
mod models;
mod tests;

use routes::route::create_router;

use colored::Colorize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    //Clear the terminal and move the cursor to top left corner
    print!("\x1B[2J\x1B[1;1H");

    startup::print_banner(); //outputs the name using ASCII
    startup::print_initializing(); //output the initializing text

    //Find the correct valid port - default to 3000
    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT must be a valid u16");

    let timer = startup::create_timer();
    startup::print_step("Loading environment", true, startup::elapsed_ms(timer));

    //Load up and connect to the database
    let timer = startup::create_timer();
    let _db = match db::init().await {
        Ok(db) => db,
        Err(e) => {
            startup::print_step("Connecting to database", false, startup::elapsed_ms(timer));
            eprintln!("{}", format!("  Error: {}", e).red());
            std::process::exit(1);
        }
    };
    startup::print_step("Connecting to database", true, startup::elapsed_ms(timer));

    let timer = startup::create_timer();
    let jwt_config = match utility::jwt::JwtConfig::from_env() {
        Ok(config) => config,
        Err(e) => {
            startup::print_step("Loading JWT config", false, startup::elapsed_ms(timer));
            eprintln!("{}", format!("  Error: {}", e).red());
            std::process::exit(1);
        }
    };
    startup::print_step("Loading JWT config", true, startup::elapsed_ms(timer));

    //Write the tables to the database if they don't exist (initial.squrl)
    let timer = startup::create_timer();
    match db::schema::init(&_db).await {
        Ok(_) => {
            startup::print_final_step("Initializing schema", true, startup::elapsed_ms(timer));
            let _ = db::queries::server_logs::log_startup(&_db, startup::elapsed_ms(timer) as i64)
                .await;
        }
        Err(e) => {
            startup::print_final_step("Initializing schema", false, startup::elapsed_ms(timer));
            eprintln!("{}", format!("  Error: {}", e).red());
            let _ = db::queries::server_logs::log_error(&_db, e.to_string(), 0).await;
            std::process::exit(1);
        }
    }

    startup::print_ready(port);

    //start the router
    let app = create_router(_db.clone(), jwt_config);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let server = axum::serve(listener, app);

    //run the tests to ensure the server is fully functional without errors
    if std::env::var("ENABLE_TESTS").unwrap_or_else(|_| "true".to_string()) == "true" {
        //spawn server in background
        //to ensure the server runs smoothly at the same time as running the tests
        //the sever is spawned in its own thread whilst the tests run on main thread
        let server_task = tokio::spawn(async move {
            server.await
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        tests::run_all_tests().await;

        //Prettify the Ctrl+C shutdown and log the shutdown
        let shutdown = async {
            let _ = tokio::signal::ctrl_c().await;
        };

        tokio::select! {
            _ = server_task => {},
            _ = shutdown => {
                println!("\nShutting down...");
                let _ = db::queries::server_logs::log_shutdown(&_db, startup::elapsed_ms(timer) as i64).await;
            }
        }
    } else {
        let shutdown = async {
            let _ = tokio::signal::ctrl_c().await;
        };

        tokio::select! {
            _ = server => {},
            _ = shutdown => {
                println!("\nShutting down...");
                let _ = db::queries::server_logs::log_shutdown(&_db, startup::elapsed_ms(timer) as i64).await;
            }
        }
    }
}
