use adventure_generator::game_system::game;
use adventure_generator::quest::*;

fn main() {
   //open menu

   let game = game::Game::new();
   
   game.get_state(game::State::Create);

   //generate file
   //read file
   //tokenize file
   //parse file
   //generate quest
}

