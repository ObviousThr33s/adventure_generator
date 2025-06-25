use std::{fmt::format, process::exit};

use crate::game_system::{animations::{print_frames_with_delay, shimmer::{gen_tv_static}}, command_system::CommandSystem, game::{classify_input::Classify, write::Writer}};
use rand::distr::Alphanumeric;
use rand::Rng;

pub mod write;
pub mod classify_input;
pub mod dialogue;

#[derive(Clone)]
pub struct Game {
	energy:i32,
	input:String,
	scenes:Vec<(String, fn(&mut Game))>,
	current_output: Vec<(String, f32)>
}

impl Game{

	pub fn new() -> Self {
		Game {
			energy:100,
			input:"monologue".to_string(),
			scenes:vec![("exit".to_string(), Self::exit_clean),
						("dialogue".to_string(),Self::dialogue),
						("question".to_string(), Self::monologue),
						("energy".to_string(), Self::energy_check),
						("data".to_string(), Self::data_dump),
						("question place".to_string(), Self::scene1)],
			current_output: Vec::new(),
		}
	}

	pub fn init() {
		
		let mut game = Game::new();
		let classiy_input:Classify = Classify::new();
		let scenes = &game.scenes.clone();

		Self::scene0(&mut game);

		loop {
			game.input = CommandSystem::read_input();

			Self::watcher(&classiy_input, scenes, &mut game);
			game.energy -= 5;
			
			if game.energy <= 5 {
				Writer::write("My energy is getting low...");
				Writer::write("Shutting down...");
				print_frames_with_delay(gen_tv_static(5), 3);	
				exit(0x0);
			}
			
		}
	}

	fn watcher(classify_input:&Classify, scenes:&Vec<(String, fn(&mut Game))>, g:&mut Game) {

		let labels: Vec<String> = scenes.iter().map(|(label, _)| label.clone()).collect();

		let mut output = classify_input.classify(&g.input, labels);

		// Sort output in descending order by confidence score
		output.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
		
		g.current_output = output.clone();

		print!("{:?}\n", output.clone());

		// Get the highest confidence label
		let label = if !g.current_output.is_empty() {
			output[0].0.clone()
		} else {
			"unknown".to_string()
		};

		Self::map_to_outcome(scenes, g, label);
	}

	fn energy_check(g: &mut Game) {
		println!("[Energy at:{}] ", g.energy);
	}
	
	fn monologue(g: &mut Game){
		Self::scene0(g);
	}
	
	fn dialogue(g: &mut Game){
		let text = dialogue::Dialogue::start_dialogue(
			g.energy,
			 g.input.clone());
		Writer::write(&text);
	}

	fn data_dump (g:&mut Game) {
		let labels: Vec<String> = g.scenes.iter().map(|(label, _)| label.clone()).collect();
		let printed_labels = format!("\n\n{:#?}", labels);
		
		Writer::write(&printed_labels);
	}

	fn map_to_outcome(outcomes:&Vec<(String, fn(g:&mut Game))>, g:&mut Game, label:String){
		for o in outcomes {
			if o.0.contains(&label) {
				g.energy -= 10;
				(o.1)(g)
			}
		}
	}
	fn exit_clean(_g: &mut Game){
		exit(0x0);
	}


	fn scene0(g: &mut Game) {
		print_frames_with_delay(gen_tv_static(10), 15);
		Writer::write("System cleared...");
		Writer::write("Ready...");
		Writer::write("....");
		Writer::write("hello");
	}

	fn scene1(g: &mut Game) {
		fn base0(){
			Writer::write("Where am I?");
			Writer::write("....");
			Writer::write("hello?");
		}

		fn base1(){
			Writer::write(&Game::get_random_string(rand::random_range(0..100)));
		}

		fn base2(){
			Writer::write("help");
			Writer::write(format!("im {}", Game::get_random_string(7)).as_str());
		}
		let cases:Vec<fn()> = vec![base0, base1, base2];

		let mut rng = rand::thread_rng();
		let idx = rng.gen_range(0..cases.len());
		cases[idx]();
		
		
	}

	fn get_random_string(count:usize) -> String{
		let rand_string: String = rand::thread_rng()
				.sample_iter(&Alphanumeric)
				.take(count)
				.map(char::from)
				.collect();

		rand_string
	}
}