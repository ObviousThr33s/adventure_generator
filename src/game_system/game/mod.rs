use std::process::exit;
use std::io;
use rust_bert::pipelines::question_answering::{self, QaInput};
use rust_bert::pipelines::question_answering::QuestionAnsweringModel;
use self::token_data::TokenData;

pub mod queue;
pub mod token_data;
pub mod token;

pub struct Game{
    queue: queue::Queue,
    state:State,
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
            queue: queue::Queue::new(),
            state:State::Create,
        }
    }

    pub fn get_state(&self, state:State) -> () {
        let mut t:token::Token = token::Token::new(0,
            token::TokenType::Objective,
            token::Position::new(0,0),
            token_data::TokenData{
                objective_prompt: String::from(""),
                giver_prompt: String::from(""),
                reward_prompt: String::from("")
            });

        match state {
            State::Create => {
                t = token::Token::new(0,
                    token::TokenType::Objective,
                    token::Position::new(0,0),
                    token_data::TokenData{
                        objective_prompt: self.create_objective(),
                        giver_prompt: self.create_giver(),
                        reward_prompt: self.create_reward()
                    });
                
                self.get_state(State::Write);

            }
            State::Write => {
                self.write(t);
                self.get_state(State::Play);
            }
            State::Play => {
                self.play();
                self.get_state(State::End);
            }
            State::End => {
                exit(0x0);
            }
            _ => {
                exit(0x1);
            }
        };
    }

    pub fn create_objective(&self) -> String {
        println!("Objective Prompt: ");
        let s:String = Self::read_line();

        let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();
        let qa_results = qa_model.predict(&[QaInput{ 
            question: s, 
            context: String::from("I dont know where I am, I cannot see, where am I?")}], 1, 32);
        
        qa_results[0][0].answer.clone()
    }
    

    pub fn create_giver(&self) -> String {
        println!("Giver Prompt: ");
        let s:String = Self::read_line();
        
        let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();
        let qa_results = qa_model.predict(&[QaInput{ 
            question: s, 
            context: String::from("I dont know where I am, I cannot see, where am I?")}], 1, 32);
        
        qa_results[0][0].answer.clone()
    }

    pub fn create_reward(&self) -> String {
        println!("Reward Prompt: ");
        let s:String = Self::read_line();
        
        let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();
        let qa_results = qa_model.predict(&[QaInput{ 
            question: s, 
            context: String::from("I dont know where I am, I cannot see, where am I?")}], 1, 32);
        
        qa_results[0][0].answer.clone()
    }

    pub fn write(&self, t:token::Token){
        println!("The token is: {}", t.value.to_string());
    }

    pub fn play(&self) {
    }


    fn read_line() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }
}