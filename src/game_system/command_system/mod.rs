use std::io::{self, BufRead, Write};
use std::fmt::Debug;

use dialoguer::console::Key;
use json::{parse, value};
use tch::COptimizer;

pub struct CommandSystem;
#[derive(Debug, Clone)]
pub enum CommandType {
	EXIT,
	UNKNOWN,
	HELP
}

impl CommandSystem {
	fn new() -> Self {
		CommandSystem
	}

	fn load_commands_from_file(filename: &str) -> Result<Vec<(Vec<String>, CommandType)>, std::io::Error> {
		// Read the file content
		let file_content = match std::fs::read_to_string("./commands.json") {
			Ok(content) => content,
			Err(e) => return Err(e),
		};
		// Parse the JSON content
		let json_data = match parse(&file_content) {
			Ok(data) => data,
			Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
		};

		let mut commands: Vec<(Vec<String>, CommandType)> = Vec::new();
		
		// Check if the JSON data is an object
		// Iterate over the object to extract command types and their aliases
		if let json::JsonValue::Object(obj) = json_data {
			for (key, value) in obj.iter() {
				// Convert string key to CommandType
				// Command keys are expected to be in uppercase
				// add new command key bindings here and in commands.json
				let cmd_type = match key {
					"EXIT" => CommandType::EXIT,
					"UNKNOWN" => CommandType::UNKNOWN,
					"HELP" => CommandType::HELP,
					_ => CommandType::UNKNOWN,
				};
				
				// Process all values for this command type
				if let json::JsonValue::Array(cmd_array) = &value {
					for cmd in cmd_array {
						if let Some(cmd_str) = cmd.as_str() {
							let aliases: Vec<String> = cmd_str.split_whitespace()
								.map(|s| s.to_string())
								.collect();
							commands.push((aliases, cmd_type.clone()));
						}
					}
				}
			}
		}
	

		Ok(commands)
	}

	pub fn read_input() -> String {
		// Simulate reading input from the user
		print!("<<:");
		// Flush to ensure prompt is displayed before waiting for input
		std::io::stdout().flush().expect("Failed to flush stdout");
		let mut input = String::new();
		io::stdin().lock().read_line(&mut input).expect("Failed to read line");
		// Remove the trailing newline character
		input.trim().to_string()
	}

	pub fn parse_command(input: &str) -> CommandType {
		// Simulate parsing the command
		// Convert input to lowercase to make command parsing case-insensitive
		let input = input.to_lowercase();

		// Load commands from a configuration file
		let commands = match Self::load_commands_from_file("commands.json") {
			Ok(cmds) => cmds,
			Err(_) => {
			// Fallback to default commands if file loading fails
			vec![(vec!["exit".to_string(), "quit".to_string()], CommandType::EXIT)]
			}
		};

		let command_type = Self::parser(input.to_owned(), commands);

		command_type
	}

	fn parser(input:String, mask:Vec<(Vec<String>, CommandType)>) -> CommandType {
		let mut command_type = CommandType::UNKNOWN;
		let input = input.to_lowercase();

		for (cmds, cmd_type) in mask {
			if cmds.iter().any(|cmd| input == *cmd) {
				command_type = cmd_type;
				break;
			}
		}

		command_type
	}
}