use std::{io::{self, Write}, thread};

pub struct Writer;

impl Writer {
	pub fn new() -> Writer {
		Writer {}
	}
	
	pub fn read_file(path: String) -> io::Result<String> {
		std::fs::read_to_string(path)
	}

	pub fn write(s:&str) -> () {
		let text = s;

		print!("<<:");
		let text = if let Some(index) = text.find('\n') {
			text[index + 1..].to_string()
		} else {
			text.to_string()
		};

		for c in text.chars() {
			print!("{}", c);
			io::stdout().flush().unwrap(); // Ensure character is displayed immediately
			let delay = rand::random::<u64>() % 100; // Random delay between 0-99ms
			thread::sleep(std::time::Duration::from_millis(delay));
		}
		println!(); // Add a newline at the end
		
	}
	
	pub fn read_line() -> String {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).expect("Failed to read line");
		input.trim().to_string()
	}
}