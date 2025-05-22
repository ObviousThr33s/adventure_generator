use std::process::Command;

use crate::game_system::{command_system, generators};

use super::Game;

pub struct State {
	pub prompt:String,
	pub state:GameState
}


#[derive(Clone)]
pub enum GameState{
	GeneratePrompt,
	ParseAnswer,
	GenerateResponse,
	Play,
	End
}
#[derive(Clone)]
pub enum InputLabels{
	Combat,
	Exploration,
	Puzzle,
	Other
}

impl Clone for State {
	fn clone(&self) -> Self {
		State {
			prompt: self.prompt.clone(),
			state: self.state.clone(),
		}
	}
}

impl State {
	pub fn new(prompt:String, state:GameState) -> State {
		State {
			prompt,
			state
		}
	}

	pub fn get_prompt(&self) -> String {
		self.prompt.clone()
	}
	
	pub fn get_state(&self) -> GameState {
		self.state.clone()
	}
	
	pub fn set_prompt(&mut self, prompt:String) {
		self.prompt = prompt;
	}
	
	pub fn set_state(&mut self, state:GameState) {
		self.state = state;
	}
	
	pub fn generate_prompt() -> String {
		//String::from("You are a player in a text-based game. You can explore, fight, or solve puzzles. What do you want to do?")
		String::from("No prompt available.")
	}
		
	pub fn generate_response(self) -> (InputLabels, String) {
		let response_generator:generators::classify_input::Classify = 
			generators::classify_input::Classify::new();

		let labels = vec![
			String::from("combat"),
			String::from("exploration"),
			String::from("puzzle"),
			String::from("other")
		];

		response_generator.classify(&self.prompt, labels.clone());

		for (label, score) in response_generator.classify(&self.prompt, labels.clone()) {
			println!("{}: {}", label, score);
		}
		
		// Find the string with the highest score
		let classified = response_generator.classify(&self.prompt, labels);

		// Default response if no classification is found
		let mut highest_score = 0.0;
		let mut highest_label = String::from("other");

		for (label, score) in classified {
			if score > highest_score {
				highest_score = score;
				highest_label = label;
			}
		}

		// Convert string to InputLabels enum variant
		match highest_label.as_str() {
			"combat" => (InputLabels::Combat, highest_label),
			"exploration" => (InputLabels::Exploration, highest_label),
			"puzzle" => (InputLabels::Puzzle, highest_label),
			_ => (InputLabels::Other, highest_label),
		}
	}

	pub fn parse_answer_as_command(game: &mut Game) -> String {
		let command = command_system::CommandSystem::read_input();
		let parsed_command = command_system::CommandSystem::parse_command(&command);
		
		//Add a match statement to handle the parsed command
		//this will allow for the command to be executed
		match parsed_command {
			
			command_system::CommandType::EXIT => {
				game.game_state.set_state(GameState::End);
				"Exiting the game...".to_string()
			}

			//Return the text of the command as a string if it is not a command
			//this can then be passed to the response generator
			command_system::CommandType::UNKNOWN => {
				game.game_state.set_state(GameState::GenerateResponse);
				command
			}
			
			command_system::CommandType::HELP => {
				game.game_state.set_state(GameState::GeneratePrompt);
				"Available commands: EXIT, HELP,\nAvailable actions: try exploring or fighting.".to_string()
			}
		}
	}
}