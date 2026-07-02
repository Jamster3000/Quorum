use crate::cli::AdminSession;
use crate::db::schema;
use colored::Colorize;
use quorum_core::db::{self as core_db, DB};
use std::sync::{Arc, Mutex};
use quorum_core::utility::std::typewriter_println;
use dialoguer::Input;
use quorum_core::utility::secrets::{prompt_passphrase, secrets_exist};
use quorum_core::startup;

/// Handles user signup
///
/// Allows anyone with access to the server to be able to signup and create an account.
/// When this is called it prompts for username, email (optional) and password one by one, where password is typed hidden.
///
/// # Arguments
/// * `db` - A reference to the database connection.
///
/// # Example
/// ```rust
/// signup(&db).await; // Prompts for username, password, and optional email to create a new user
/// ```
pub async fn signup(db: &DB) {
    let username = prompt("  Username: ");
    let password = prompt_password("  Password: ");
    let email = prompt("  Email (optional, leave blank): ");

    let email = if email.is_empty() { None } else { Some(email) };

    match crate::db::queries::auth::signup_user(db, &username, email.as_deref(), &password, None)
        .await
    {
        Ok(user) => println!(
            "{}",
            format!("  Created user: {} ({:?})", user.username, user.id.key).green()
        ),
        Err(e) => println!("{}", format!("  Failed to create user: {}", e).red()),
    }
}

/// Handles user login
///
/// Prompts for username and password, and verifies the credentials against the database.
/// When this is called it prompts for username and password one by one, where password is typed hidden.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `session` - A reference to the admin session, used to track login state.
///
/// # Example
/// ```rust
/// login(&db, &session).await; // Prompts for username and password to log in
/// ```
pub async fn login(db: &DB, session: &Arc<Mutex<AdminSession>>) {
    {
        let sess = session.lock().unwrap();
        if sess.is_valid() {
            println!(
                "{}",
                format!(
                    "Already logged in as {}. Run server:logout first.",
                    sess.username.as_deref().unwrap_or("unknown")
                )
                .yellow()
            );
            return;
        }
    }

    let username = prompt("  Username: ");
    let password = prompt_password("  Password: ");

    match crate::db::queries::auth::verify_user_credentials(db, &username, &password).await {
        Ok(user) => {
            let mut sess = session.lock().unwrap();
            sess.login(user.username.clone(), user.is_admin);
            println!(
                "{}",
                format!("  Logged in as {}.", user.username).green().bold()
            );
            println!(
                "{}",
                format!(
                    "  Session will expire after {} minutes of inactivity.",
                    crate::cli::SESSION_TIMEOUT_MINS
                )
                .dimmed()
            );
        }
        Err(_) => {
            println!("{}", "  Invalid username or password.".red());
        }
    }

    println!();
}

/// Handles making a user an admin
///
/// This uses a username as reference to mark a user as an admin.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `username` - The username of the user to be made an admin.
/// * `session` - A reference to the admin session, used to update the current session if the logged-in user is made an admin.
///
/// # Example
/// ```rust
/// make_admin(&db, "some_user", &session).await;
/// ```
pub async fn make_admin(db: &DB, username: &str, session: &Arc<Mutex<AdminSession>>) {
    match crate::db::queries::auth::make_admin(db, username).await {
        Ok(_) => {
            println!("{}", format!("  {} is now an admin.", username).green());

            let mut sess = session.lock().unwrap();
            if sess.username.as_deref() == Some(username) {
                sess.update_is_admin(true);
            }
        }
        Err(e) => println!("{}", format!("  Failed: {}", e).red()),
    }
}

pub async fn confirm_and_delete(db: &DB, shutdown_tx: &tokio::sync::watch::Sender<bool>) {
    println!();
    let _ = typewriter_println(&format!(
        "{}",
        "Are you sure?\nThis will delete the whole database and all data stored, restarting the whole database from scratch (y/n)".yellow()
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
        "Enter passphrase to confirm database deletion...".cyan().bold()
    ))
    .map_err(|e| e.to_string());

    match prompt_passphrase() {
        Ok(passphrase) => {
            if let Err(e) = perform_reset(db, &passphrase).await {
                eprintln!("{}", format!("Failed to reset database: {e}").red());
                return;
            }
            println!(
                "{}",
                "Database reset complete. The server is shutting down...".green()
            );
            let _ = shutdown_tx.send(true);
        }
        Err(e) => {
            eprintln!("Failed to read passphrase: {}", e);
        }
    }
}

async fn perform_reset(db: &DB, passphrase: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Verify passphrase is correct by attempting to reload config
    quorum_core::utility::config::Config::load_with_passphrase(passphrase)?;

    let config = quorum_core::utility::config::Config::get();
    let path = std::path::Path::new(&config.surreal_data_path);

    // Delete the database directory
    if path.exists() {
        std::fs::remove_dir_all(path)?;
    }

    // Recreate it
    std::fs::create_dir_all(path)?;

    // Reinitialize the database
    let timer = startup::create_timer();
    let new_db = quorum_core::db::init().await?;
    startup::print_step("Opening database", true, startup::elapsed(timer));

    let timer = startup::create_timer();
    schema::init(&new_db).await?;
    startup::print_step("Initializing schema", true, startup::elapsed(timer));

    Ok(())
}

/// Prompts the user for input with a given label and returns the trimmed input as a String.
///
/// # Arguments
/// * `label` - The label to display before the input prompt.
///
/// # returns
/// A String containing the user's input, trimmed of whitespace.
fn prompt(label: &str) -> String {
    print!("{}", label.white());
    std::io::Write::flush(&mut std::io::stdout()).ok();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input.trim().to_string()
}

/// Prompts the user for a password with a given label and returns the input as a String.
///
/// # Arguments
/// * `label` - The label to display before the password prompt.
///
/// # returns
/// A String containing the user's password input, or an empty string if reading the password fails.
fn prompt_password(label: &str) -> String {
    print!("{}", label.white());
    std::io::Write::flush(&mut std::io::stdout()).ok();

    rpassword::read_password().unwrap_or_default()
}