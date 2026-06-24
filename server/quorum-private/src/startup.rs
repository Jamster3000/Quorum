//! Server startup printing initiliation

//! Provides public functions to be used for neat and pretty output during server startup.
//! Including the ASCII banner, step-by-step progress logging, and timing information.

use colored::*;
use std::thread;
use std::time::{Duration, Instant};

/// Display the ASCII art banner
///
/// Prints a centered, coloured ASCII art banner.
/// The banner is used at the very start of initializing the server to give it a unique and identifiable look in the terminal.
/// The ASCII art is generated from https://patorjk.com/software/taag/#p=display
///
/// # Example
/// ```rust
/// print_banner();
/// ```
pub fn print_banner() {
    println!("{}", "═".repeat(75).cyan());
    //ASCII art generated with https://patorjk.com/software/taag/#p=display
    println!(
        r#"
     ██████
   ███░░░░███
  ███    ░░███ █████ ████  ██████  ████████  █████ ████ █████████████
 ░███     ░███░░███ ░███  ███░░███░░███░░███░░███ ░███ ░░███░░███░░███
 ░███   ██░███ ░███ ░███ ░███ ░███ ░███ ░░░  ░███ ░███  ░███ ░███ ░███
 ░░███ ░░████  ░███ ░███ ░███ ░███ ░███      ░███ ░███  ░███ ░███ ░███
  ░░░██████░██ ░░████████░░██████  █████     ░░████████ █████░███ █████
    ░░░░░░ ░░   ░░░░░░░░  ░░░░░░  ░░░░░       ░░░░░░░░ ░░░░░ ░░░ ░░░░░
    "#
    );
    println!("{}", "═".repeat(75).cyan());
}

/// Display the "Initializing..." startup message
///
/// Displays the text in bold yellow text waiting 300ms before continuing.
///
/// # Example
/// ```rust
/// print_initializing();
/// ```
pub fn print_initializing() {
    println!("\n{}", "Initializing...".yellow().bold());
    thread::sleep(Duration::from_millis(300));
}

/// Display a single initialization step with status and timing
///
/// Prints a formatted line showing a step name, success/failure status,
/// and elapsed tim in milliseconds.
///
/// # Arguments
/// * `step` - A string describing the initialization step being performed.
/// * `success` - A boolean indicating whether the step succeeded (true) or failed (false).
/// * `duration_ms` - The time taken to complete the step, in milliseconds.
///
/// # Example
/// ```rust
/// let timer = create_timer();
/// print_step("Initializing database", true, elapsed_ms(timer));
/// ```
pub fn print_step(step: &str, success: bool, duration: Duration) {
    let status = if success { "✓".green() } else { "✗".red() };
    let timing = format!("({:?})", duration).dimmed();
    println!("{}{} {} {}", "  ├─ ".blue(), step.white(), status, timing);
    thread::sleep(Duration::from_millis(200));
}

/// Display the final "Server ready" message with the server URL
///
/// Prints a formatted message indicating that the server is ready, including the URL and port number.
/// The URL is displayed in green and bold for emphasis, and a note about stopping the server is shown in dimmed text.
///
/// # Arguments
/// * `port` - The port number on which the server is running.
///
/// # Example
/// ```rust
/// print_ready(8080);
/// ```
pub fn print_ready(port: u16) {
    println!(
        "\n{}",
        format!("Server ready at http://127.0.0.1:{}", port)
            .green()
            .bold()
    );
    println!("{}", "Press Ctrl+C to stop\n".dimmed());
}

/// Create a timer for measuring elapsed time
///
/// # Returns
/// An `Instant` representing the current time, which can be used to measure elapsed time for initialization steps.
///
/// # Example
/// ```rust
/// let timer = create_timer();
/// let elapsed = elapsed_ms(timer);
/// println!("Step completed in {} ms", elapsed);
/// ```
pub fn create_timer() -> Instant {
    Instant::now()
}

/// Calculate elapsed time as a `Duration` from a given timer
///
/// # Arguments
/// * `timer` - An `Instant` representing the start time of an operation.
///
/// # Returns
/// The elapsed time as a `Duration`.
///
/// # Example
/// ```rust
/// let timer = create_timer();
/// let elapsed = elapsed(timer);
/// println!("Operation completed in {:?}", elapsed);
/// ```
pub fn elapsed(timer: Instant) -> Duration {
    timer.elapsed()
}
