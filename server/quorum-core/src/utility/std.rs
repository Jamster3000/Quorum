//! This file includes some basic std::io functionality
//! including outputting text as a typewriter effect and waiting for user input to continue.

use crate::startup;
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

/// Waits for the user to press Enter to continue. Optionally clears the screen and reprints the banner.
///
/// # Arguments
/// * `clear_screen` - If true, clears the terminal screen after pressing Enter.
/// * `reprint_banner` - If true, reprints the banner after clearing the screen.
///
/// # Example
/// ```
/// let clear_screen = true;
/// let reprint_banner = true;
/// press_enter_to_continue(clear_screen, reprint_banner);
/// ```
pub fn press_enter_to_continue(clear_screen: bool, reprint_banner: bool) {
    use std::io::{self, Write};
    print!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if clear_screen {
        print!("\x1B[2J\x1B[1;1H");
    }

    if reprint_banner {
        startup::print_banner();
    }
}

/// Prints the given text to the console with a typewriter effect, where each character is printed with a delay.
///
/// # Arguments
/// * `text` - The text to be printed with the typewriter effect.
///
/// # Returns
/// * `io::Result<()>` - Returns an `io::Result` indicating success or failure of the printing operation.
///
/// # Example
/// ```
/// typewriter_println(&format!("{}","Hello, World!"))
/// ```
pub fn typewriter_println(text: &str) -> io::Result<()> {
    let mut stdout = io::stdout();
    let delay_ms: u64 = 35;

    for ch in text.chars() {
        print!("{ch}");
        stdout.flush()?;
        thread::sleep(Duration::from_millis(delay_ms));
    }

    println!();
    Ok(())
}
