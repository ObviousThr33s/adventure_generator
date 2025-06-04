use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;


#[allow(dead_code)]
pub struct Classify {
	model:ZeroShotClassificationModel,
}

impl Classify {
	pub fn new() -> Classify {
		let model = ZeroShotClassificationModel::new(Default::default()).unwrap();
		Classify {
			model
		}
	}
	
	pub fn classify(&self, prompt:&str, labels:Vec<String>) -> Vec<(String, f32)> {
		// Convert Vec<String> to Vec<&str>
		let labels_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
		let output = 
			self.model.predict_multilabel(&[prompt], labels_refs, None, 128).unwrap();
		// Convert the first result from Vec<Vec<Label>> to Vec<(String, f32)>
		output[0].iter().map(|label| (label.text.clone(), label.score as f32)).collect()
	}
}