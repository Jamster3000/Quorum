use colored::*;
use std::thread;
use std::time::{Duration, Instant};

pub fn print_banner() {
    println!("{}", "═".repeat(75).cyan());
    //ASCII art generated with https://patorjk.com/software/taag/#p=display
    println!(r#"
     ██████                                                            
   ███░░░░███                                                          
  ███    ░░███ █████ ████  ██████  ████████  █████ ████ █████████████
 ░███     ░███░░███ ░███  ███░░███░░███░░███░░███ ░███ ░░███░░███░░███
 ░███   ██░███ ░███ ░███ ░███ ░███ ░███ ░░░  ░███ ░███  ░███ ░███ ░███
 ░░███ ░░████  ░███ ░███ ░███ ░███ ░███      ░███ ░███  ░███ ░███ ░███
  ░░░██████░██ ░░████████░░██████  █████     ░░████████ █████░███ █████
    ░░░░░░ ░░   ░░░░░░░░  ░░░░░░  ░░░░░       ░░░░░░░░ ░░░░░ ░░░ ░░░░░
    "#);
    println!("{}", "═".repeat(75).cyan());
}

pub fn print_initializing() {
    println!("\n{}", "Initializing...".yellow().bold());
    thread::sleep(Duration::from_millis(300));
}

pub fn print_step(step: &str, success: bool, duration_ms: u128) {
    let status = if success { "✓".green() } else { "✗".red() };
    let timing = format!("({:.0}ms)", duration_ms).dimmed();
    println!("{}{} {} {}", "  ├─ ".blue(), step.white(), status, timing);
    thread::sleep(Duration::from_millis(200));
}

pub fn print_final_step(step: &str, success: bool, duration_ms: u128) {
    let status = if success { "✓".green() } else { "✗".red() };
    let timing = format!("({:.0}ms)", duration_ms).dimmed();
    println!("{}{} {} {}", "  └─ ".blue(), step.white(), status, timing);
    thread::sleep(Duration::from_millis(200));
}

pub fn print_ready(port: u16) {
    println!(
        "\n{}",
        format!("Server ready at http://127.0.0.1:{}", port)
            .green()
            .bold()
    );
    println!("{}", "Press Ctrl+C to stop\n".dimmed());
}

pub fn create_timer() -> Instant {
    Instant::now()
}

pub fn elapsed_ms(timer: Instant) -> u128 {
    timer.elapsed().as_millis()
}