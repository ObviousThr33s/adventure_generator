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
}

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
}