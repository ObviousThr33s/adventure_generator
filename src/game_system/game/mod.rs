use std::{fmt::format, process::exit};

use crate::game_system::{animations::{print_frames_with_delay, shimmer::{gen_tv_static}}, command_system::CommandSystem, game::{classify_input::Classify, write::Writer}};

pub mod write;
pub mod classify_input;

#[derive(Clone)]
pub struct Game {
	hp:i32,
	energy:i32,
	joy:i32,
	scenes:Vec<(String, fn(&mut Game))>
}

impl Game{

	pub fn new() -> Self {
		Game {
			hp:100,
			energy:100,
			joy:100,
			scenes:vec![("exit".to_string(), Self::exit_clean),
						("dialogue".to_string(),Self::scene0),
						("question".to_string(),Self::scene1),
						("energy".to_string(), Self::energy_check),
						("data".to_string(), Self::data_dump)]
		}
	}

	pub fn init() {
		
		let mut game = Game::new();
		let classiy_input:Classify = Classify::new();
		let scenes = &game.scenes.clone();

		loop {
			let input = CommandSystem::read_input();
			game.energy -= 5;
			Self::watcher(&classiy_input, input, scenes, &mut game);
			game.energy -= 5;
			
			if game.energy <= 10 {
				Writer::write("My energy is getting low...");
				Writer::write("Shutting down...");
				print_frames_with_delay(gen_tv_static(5), 3);	
				exit(0x0);
			}
			
		}
	}

	fn watcher(classify_input:&Classify, input:String, scenes:&Vec<(String, fn(&mut Game))>, g:&mut Game) {

		let labels: Vec<String> = scenes.iter().map(|(label, _)| label.clone()).collect();

		let mut output = classify_input.classify(&input, labels);

			// Sort output in descending order by confidence score
			output.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
			
			print!("{:?}\n", output.clone());

			// Get the highest confidence label
			let label = if !output.is_empty() {
				output[0].0.clone()
			} else {
				"unknown".to_string()
			};

			Self::map_to_outcome(scenes, g, label);
	}


	fn energy_check(g: &mut Game) {
		g.energy -= 10;
		println!("[Energy at:{}] ", g.energy);
	}

	fn scene0(g: &mut Game) {
		g.energy -= 10;
		print_frames_with_delay(gen_tv_static(10), 15);
		Writer::write("System cleared...");
		Writer::write("Ready...");
		Writer::write("....");
		Writer::write("hello");
	}

	fn scene1(g: &mut Game) {
		g.energy -= 10;
		Writer::write("Where am I?");
		Writer::write("....");
		Writer::write("hello?");
	}

	fn data_dump (g:&mut Game) {
		let labels: Vec<String> = g.scenes.iter().map(|(label, _)| label.clone()).collect();
		let printed_labels = format!("{:#?}", labels);

		Writer::write(&printed_labels);
	}

	fn map_to_outcome(outcomes:&Vec<(String, fn(g:&mut Game))>, g:&mut Game, label:String){
		for o in outcomes {
			if o.0.contains(&label) {
				(o.1)(g)
			}
		}
	}
	fn exit_clean(_g: &mut Game){
		exit(0x0);
	}

}