use std::process::exit;
use std::io;

use rust_bert::{gpt2::GPT2Generator, pipelines::{generation_utils::{GenerateOptions, GeneratedTextOutput, LanguageGenerator},text_generation::TextGenerationModel}};

pub mod queue;
pub mod token_data;
pub mod token;

pub struct Game{
	_queue: queue::Queue,
	_state:State,
}

pub enum State {
	Create,
	Write,
	Play,
	End,
}

//seek the word

impl Game {
	pub fn new() -> Game {
		Game {
			_queue: queue::Queue::new(),
			_state:State::Create,
		}
	}

	pub fn get_state(&mut self, state:State) -> () {
		
		match state {
			State::Create => {
				println!("{:?}", self.create_objective());
				self.get_state(State::Write);
			}
			State::Write => {
				// /self.write(t.clone());
			}
			State::Play => {
				self.play();
			}
			State::End => {
				exit(0x0);
			}
		};
	}

	pub fn create_objective(&mut self) -> Vec<GeneratedTextOutput> {
		let model = GPT2Generator::new(Default::default ())
			.expect("Could not create generator");

		let generate_options = GenerateOptions {
			max_length: Some(30),
			..Default::default()
		};

		let output = model.generate(Some(&["generate a name"]), Some(generate_options));
		output.unwrap()
	}

	pub fn write(&mut self, t:token::Token){
		println!("{}", t.value.to_string());
		self.get_state(State::Play);
	}

	pub fn play(&mut self) -> () {
		self.get_state(State::End);
	}


	fn _read_line() -> String {
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read line");
		input.trim().to_string()
	}
}