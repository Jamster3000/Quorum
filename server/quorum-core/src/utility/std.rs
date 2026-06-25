use crate::startup;
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

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