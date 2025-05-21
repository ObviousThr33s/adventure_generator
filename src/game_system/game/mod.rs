use std::process::exit;
use std::io::{self};

use state::{GameState, State};
use write::Writer;


pub mod write;
pub mod create;
pub mod play;

pub mod state;

pub struct Game{
	system_state:SystemState,
	game_state:State,
	tick:usize
}

pub enum SystemState {
	Create,
	Write,
	Play,
	End,
}


//seek the word

impl Game {
	pub fn new() -> Game {
		Game {
			system_state:SystemState::Write,
			game_state:State::new(
				"Generate a prompt".to_string(),
				GameState::GeneratePrompt
			),
			tick:0
		}
	}

	pub fn get_state(&mut self, state:SystemState) -> () {
		
		match state {
			SystemState::Create => {
				self.create(self.game_state.get_state());
			}
			SystemState::Write => {
				self.write(self.game_state.get_prompt());
			}
			SystemState::Play => {
				self.play();
			}
			SystemState::End => {
				exit(0x0);
			}
		};
	}

	pub fn create(&mut self, game_state:GameState) {
		
		match game_state {
			GameState::GeneratePrompt => {
				let prompt = State::generate_prompt();
				self.game_state.set_prompt(prompt);
				self.game_state.set_state(GameState::ParseAnswer);
			}
			GameState::ParseAnswer => {
				let answer = State::parse_answer(self);
				self.game_state.set_prompt(answer);
			}
			GameState::GenerateResponse => {
				let response = State::generate_response();
				self.game_state.set_prompt(response);
				self.game_state.set_state(GameState::Play);
			}
			GameState::Play => {
				self.play();
			}
			GameState::End => {
				self.get_state(SystemState::End);
			}
		}
		self.get_state(SystemState::Write);
	}


	pub fn write(&mut self, s_:String){
		let writer = write::Writer::new();

		writer.write(s_);

		self.get_state(SystemState::Create);
	}

	pub fn play(&mut self) {

		//Game play logic
		let writer = write::Writer::new();
		writer.write("Game play logic".to_string());

		self.game_state.set_state(GameState::ParseAnswer);
		self.get_state(SystemState::Create);
	}

}