use std::{io::{self, Stdout, Write}, thread, time::Duration};

use crossterm::{cursor, queue, terminal};

pub mod shimmer;

pub fn print_frames_with_delay(frames: Vec<Vec<char>>, delay_ms: u64) {
	let mut stdout = io::stdout();
	
	for frame in frames {
		// Clear the entire terminal
		queue!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
		// Print the frame
		queue!(stdout, cursor::MoveTo(0,0)).unwrap();
		stdout.flush().unwrap();
		thread::sleep(Duration::from_millis(delay_ms));
		
		for ch in frame {
			print!("{}", ch);
		}
		
		// Wait for the specified delay
		thread::sleep(Duration::from_millis(delay_ms));
	}

	queue!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
}