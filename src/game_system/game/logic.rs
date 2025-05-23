use rand::Rng;

use super::{game_entity, state::InputLabels};

pub(crate) struct Logic {
	pub entities: Vec<game_entity::Entity>,
	target:usize
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
			target: self.target,
		}
	}
}

impl Logic {
	pub fn new() -> Self {
		Logic {
			entities: Vec::new(),
			target: 1,
		}
	}

	//needs hooks to built out game system (entity manager, etc)
	pub fn update(&mut self, play_state: &mut InputLabels, tick: usize) {

		

		match play_state {
			InputLabels::Combat => {
				let attack_type = AttackType::Physical; // Example attack type
				self.attack(attack_type, 0, self.entities.len()-1);
			}
			InputLabels::Exploration => {
				self.entities.push(game_entity::Entity { id: tick, name: format!("Monster {}", self.target), health: 50, attack: 8, defense: 3 });
				self.target += 1;
				println!("Exploring... Found: {}", self.entities.last().unwrap().get_name());
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

	pub fn attack(&mut self, attack_type: AttackType, attacker_id: usize, target_id: usize) {
		
		
		let damage = self.roll(attack_type.clone());

		let attacker_idx = attacker_id;
		let target_idx = target_id;

		if target_idx == 0 {

			println!("{:?}", self.entities);

			println!("Go explore the world!");
			return;
		}

		let (attacker, target) = if attacker_idx < target_idx {
			let (left, right) = self.entities.split_at_mut(target_idx);
			(&mut left[attacker_idx], &mut right[0])
		} else if attacker_idx > target_idx {
			let (left, right) = self.entities.split_at_mut(attacker_idx);
			(&mut right[0], &mut left[target_idx])
		} else {
			println!("Go explore the world!");
			return;
		};

		if target.get_health() <= 0 {
			println!("{} is already defeated!", target.get_name());
			return;
		}
		
		println!("{}'s health before attack: {}", target.get_name(), target.get_health());
		let attack_message = match attack_type {
			AttackType::Physical => format!("{} attacks {} for {} damage!", attacker.get_name(), target.get_name(), damage),
			AttackType::Magical => format!("{} casts a spell on {} for {} damage!", attacker.get_name(), target.get_name(), damage),
		};

		target.set_health(target.get_health() - damage as i32);

		println!("{}", attack_message);
		println!("{}'s health after attack: {}", target.get_name(), target.get_health());
		if target.get_health() <= 0 {
			println!("{} has been defeated!", target.get_name());
			self.entities.remove(target_idx);
		} else {
			target.set_health(target.get_health());
		}
	}

}