use rust_bert::{gpt2::GPT2Generator, pipelines::generation_utils::{GenerateOptions, LanguageGenerator}};

#[allow(dead_code)]
pub struct BasicGenerator {
	model:GPT2Generator,
}

impl BasicGenerator {
	pub fn new() -> BasicGenerator {
		let model = GPT2Generator::new(Default::default()).unwrap();
		BasicGenerator {
			model
		}
	}
	
	pub fn generate(&self, prompt:&str) -> String {

		let generate_options = GenerateOptions {
			max_length: Some(30),
			..Default::default()
		};

		let output = self.model.generate(Some(&[prompt]), Some(generate_options));
		output.unwrap()[0].text.clone()
	}
}