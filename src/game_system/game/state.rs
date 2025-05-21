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
		String::from("Generate a prompt")
	}
		
	pub fn generate_response() -> String {
		let response_generator:generators::basic_generator::BasicGenerator = 
			generators::basic_generator::BasicGenerator::new();

		response_generator.generate("Generate a response")
	}

	pub fn parse_answer(game: &mut Game) -> String {
		let command = command_system::CommandSystem::read_input();
		let parsed_command = command_system::CommandSystem::parse_command(&command);
		
		match parsed_command {
			
			command_system::CommandType::EXIT => {
				game.game_state.set_state(GameState::End);
				"Exiting the game...".to_string()
			}

			command_system::CommandType::UNKNOWN => {
				game.game_state.set_state(GameState::GenerateResponse);
				command
			}
			
			command_system::CommandType::HELP => {
				game.game_state.set_state(GameState::GeneratePrompt);
				"Available commands: EXIT, HELP".to_string()
			}
		}
	}
}