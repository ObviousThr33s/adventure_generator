use rand::Rng;

use super::{game_entity, state::InputLabels};

pub(crate) struct Logic {
	pub entities: Vec<game_entity::Entity>,
}

#[derive(Clone)]
pub enum AttackType {
	Physical,
	Magical,
}

impl Clone for Logic {
	fn clone(&self) -> Self {
		Logic {
			entities: self.entities.clone(),
		}
	}
}

impl Logic {
	pub fn new() -> Self {
		Logic {
			entities: Vec::new(),
		}
	}

	//needs hooks to built out game system (entity manager, etc)
	pub fn update(&mut self, play_state: &mut InputLabels, tick: usize) {

		

		match play_state {
			InputLabels::Combat => {
				let attack_type = AttackType::Physical;
				let damage = self.roll(attack_type.clone());
				if self.entities.len() >= 2 {
					let (first, second) = self.entities.split_at_mut(1);
					Self::attack(damage, attack_type, &mut first[0], &mut second[0]);
					second[0].set_health(second[0].get_health() - damage);
				}
			}
			InputLabels::Exploration => {
				// Handle exploration logic
			}
			InputLabels::Puzzle => {
				// Handle puzzle logic
			}
			InputLabels::Other => {
				// Handle other logic
			}
		}
	}

	pub fn roll(&self, attack_type: AttackType) -> u32 {
		
		let damage:u32;

		let mut rng = rand::thread_rng();
		let roll = rng.gen_range(1..=20); // Roll a d20
		
		let modifier = match attack_type {
			AttackType::Physical => 5, // Example modifier for physical attack
			AttackType::Magical => 3,  // Example modifier for magical attack
		};

		match attack_type {
			AttackType::Physical => {
				// Roll for physical attack
				damage = roll + modifier;
			}
			AttackType::Magical => {
				// Roll for magical attack
				damage = roll + modifier;
			}
		}
		damage
	}

	pub fn attack(damage: u32, attack_type: AttackType, attacker: &mut game_entity::Entity, target: &mut game_entity::Entity) {
		println!("{}'s health before attack: {}", target.get_name(), target.get_health());
		let attack_message = match attack_type {
			AttackType::Physical => format!("{} attacks {} for {} damage!", attacker.get_name(), target.get_name(), damage),
			AttackType::Magical => format!("{} casts a spell on {} for {} damage!", attacker.get_name(), target.get_name(), damage),
		};

		
		println!("{}", attack_message);
		println!("{}'s health after attack: {}", target.get_name(), target.get_health() - damage);
	}

}