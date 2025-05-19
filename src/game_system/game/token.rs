use crate::game_system::game::token_data::TokenData;

pub struct Token {
    id:i128,
    pub token_type:TokenType,
    position:Position,
    pub value: TokenData,
}

pub struct Position {
    x:i128,
    y:i128,
}

impl Position {
    pub fn new(x:i128,y:i128) -> Position {
        Position {
            x: x,
            y: y,
        }
    }
    pub fn clone(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }
}
#[derive(Clone)]
pub enum TokenType {
    Objective,
    Giver,
    Reward,
}

impl Token {
    pub fn new(id:i128,token_type:TokenType,position:Position,value:TokenData) -> Token {
        Token {
            id: id,
            token_type: token_type,
            position: position,
            value: value,
        }
    }

    pub fn clone(&self) -> Token {
        Token {
            id: self.id,
            token_type: self.token_type.clone(),
            position: self.position.clone(),
            value: self.value.clone(),
        }
    }

    pub fn set_data(&mut self,val:TokenData){
        self.value = val;
    }
}