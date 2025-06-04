use std::{fmt::Write, io::{self, Stdout}, process::{exit, Stdio}};

use crossterm::{terminal, ExecutableCommand};
use rust_bert::pipelines::sequence_classification::Label;
use sorting_rs::quick_sort;

use crate::game_system::{animations::{print_frames_with_delay, shimmer::{gen_tv_static}}, command_system::CommandSystem, game::{classify_input::Classify, write::Writer}};

pub mod write;
pub mod classify_input;

pub struct Game;

impl Game{

	pub fn new() -> Self {
		Game
	}

	pub fn init() {
		let mut stdout = io::stdout();

		let classiy_input:Classify = Classify::new();

		let scenes:Vec<(String, fn())>= vec![("exit".to_string(), Self::exit_clean),
											("dialogue".to_string(),Self::scene0),
											("confused".to_string(),Self::scene1)];

		loop {
			let input = CommandSystem::read_input();

			let mut output = classiy_input.classify(&input, vec![
				"dialogue".to_string(),
				"confused".to_string(),
				"exit".to_string()
			]);

			// Sort output in descending order by confidence score
			output.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
			
			print!("{:?}\n", output.clone());

			// Get the highest confidence label
			let label = if !output.is_empty() {
				output[0].0.clone()
			} else {
				"unknown".to_string()
			};



			Self::map_to_outcome(&scenes, label);
		}
	}

	fn scene0() {
		print_frames_with_delay(gen_tv_static(10), 15);
		Writer::write("System cleared...");
		Writer::write("Ready...");
		Writer::write("....");
		Writer::write("hello");
	}

	fn scene1() {
		//print_frames_with_delay(gen_frames(10), 15, stdout);
		Writer::write("Where am I?");
		Writer::write("....");
		Writer::write("hello?");
	}

	fn map_to_outcome(outcomes:&Vec<(String, fn())>, label:String){
		for o in outcomes {
			if o.0.contains(&label) {
				(o.1)()
			}
		}
	}

	fn exit_clean(){
		exit(0x0);
	}

	//play scene
	//get input

}