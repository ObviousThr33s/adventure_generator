use std::{io, process::Output};
use once_cell::sync::Lazy;
use std::sync::Mutex;

use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};
use rust_bert::pipelines::conversation::{ConversationModel, ConversationManager};
use uuid::Uuid;

pub struct Dialogue {}

pub enum EnergyLevel {
	HIGH,
	MED,
	LOW
}

impl Dialogue {
	pub fn get_energy_level(energy:i32) -> EnergyLevel {
		if energy > 10 && energy < 50 {
			EnergyLevel::LOW
		}else if energy > 50 && energy < 80{
			EnergyLevel::MED
		}else{
			EnergyLevel::HIGH
		}
	}

	pub fn start_dialogue(energy_level:i32, user_input: String) -> String{
		use rust_bert::pipelines::conversation::{ConversationModel, ConversationManager};

		let conversation_model = ConversationModel::new(Default::default ());
		let mut conversation_manager = ConversationManager::new();

		let input = user_input.clone();
		let conversation_id = conversation_manager.create(input.as_str());
		let response = conversation_model.unwrap().generate_responses(&mut conversation_manager);
		let binding = response.unwrap();
		let output = binding.get(&conversation_id);
		output.unwrap().to_string()
	}
}