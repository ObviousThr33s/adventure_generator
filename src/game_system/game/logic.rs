struct Logic;

enum AttackType {
	Physical,
	Magical,
}

impl Logic {
	pub fn new() -> Self {
		Logic {}
	}

	pub fn update(&mut self) {
		// Update game logic here
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
	}

	pub fn attack(&mut self, damage: u32, attack_type: AttackType, attacker: &str, target: &str) {
		let attack_message = match attack_type {
			AttackType::Physical => format!("{} attacks {} for {} damage!", attacker, target, damage),
			AttackType::Magical => format!("{} casts a spell on {} for {} damage!", attacker, target, damage),
		};

		println!("{}", attack_message);
	}

}