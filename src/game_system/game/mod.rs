use std::process::exit;
use std::io::{self};

use state::{GameState, InputLabels, State};
use write::Writer;


pub mod write;
pub mod create;
pub mod play;

pub mod state;

pub mod game_entity;
pub mod logic;

pub struct Game{
	system_state:SystemState,
	game_state:State,
	play_state:InputLabels,
	tick:usize,
	logic:logic::Logic,
}

pub enum SystemState {
	Create,
	Write,
	Play,
	End,
}

impl Clone for SystemState {
	fn clone(&self) -> Self {
		match self {
			SystemState::Create => SystemState::Create,
			SystemState::Write => SystemState::Write,
			SystemState::Play => SystemState::Play,
			SystemState::End => SystemState::End,
		}
	}
}

impl Clone for Game {
	fn clone(&self) -> Self {
		Game {
			system_state: self.system_state.clone(),
			game_state: self.game_state.clone(),
			play_state: self.play_state.clone(),
			tick: self.tick,
			logic: self.logic.clone(),
		}
	}
}

impl Game {
	pub fn new() -> Game {
		Game {
			system_state:SystemState::Write,
			game_state:State::new(
				"Generate a prompt".to_string(),
				GameState::GeneratePrompt
			),
			tick:0,
			play_state:InputLabels::Other,
			logic:logic::Logic::new(),
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
				let answer = State::parse_answer_as_command(self);
				self.game_state.set_prompt(answer);
			}
			GameState::GenerateResponse => {
				let response = self.game_state.clone().generate_response();
				self.game_state.set_prompt(response.1);
				self.play_state = response.0;

				self.game_state.set_state(GameState::Play);
			}
			GameState::Play => {
				self.get_state(SystemState::Play);
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

		if self.tick == 0 {
			let attacker = game_entity::Entity::new(0, "Hero".to_string(), 100, 10, 5);
			self.logic.entities.push(attacker);
		}


		match self.play_state {
			InputLabels::Combat => {
				// Combat logic
				self.logic.update(&mut self.play_state, self.tick);
			}
			InputLabels::Exploration => {
				// Exploration logic
				self.logic.update(&mut self.play_state, self.tick);
				writer.write("Enemy found!".to_string());
			}
			InputLabels::Puzzle => {
				// Puzzle logic
				writer.write("Puzzle logic".to_string());
				self.logic.update(&mut self.play_state, self.tick);
			}
			InputLabels::Other => {
				// Other logic
				writer.write("Other logic".to_string());
				
			}
		}

		self.tick += 1;
		self.game_state.set_state(GameState::ParseAnswer);
		self.get_state(SystemState::Create);
	}

}