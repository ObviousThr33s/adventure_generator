use adventure_generator::game_system::game;

fn main() {
	let mut game = game::Game::new();

	game.get_state(game::SystemState::Create);

}
