use std::clone;

pub struct TokenData {
    pub objective_prompt:String,
    pub giver_prompt:String,
    pub reward_prompt:String,   
}

impl ToString for TokenData {
    fn to_string(&self) -> String {
        format!("Objective Prompt: {}\nGiver Prompt: {}\nReward Prompt: {}", self.objective_prompt, self.giver_prompt, self.reward_prompt)
    }
}

impl Clone for TokenData {
    fn clone(&self) -> TokenData {
        TokenData {
            objective_prompt: self.objective_prompt.clone(),
            giver_prompt: self.giver_prompt.clone(),
            reward_prompt: self.reward_prompt.clone(),
        }
    }
}

impl TokenData {
    pub fn new(objective_prompt:String,giver_prompt:String,reward_prompt:String) -> TokenData {
        TokenData {
            objective_prompt: objective_prompt,
            giver_prompt: giver_prompt,
            reward_prompt: reward_prompt,
        }
    }
}