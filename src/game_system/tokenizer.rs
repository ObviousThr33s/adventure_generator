pub struct Tokenizer {
    objective_prompt:String,
    giver_prompt:String,
    reward_prompt:String,   
}

impl Tokenizer {
    pub fn new(objective_prompt:String,giver_prompt:String,reward_prompt:String) -> Tokenizer {
        Tokenizer {
            objective_prompt: objective_prompt,
            giver_prompt: giver_prompt,
            reward_prompt: reward_prompt,
        }
    }
}