pub struct Entity {
	pub id: usize,
	pub name: String,
	pub health: i32,
	pub attack: u32,
	pub defense: i32,
}

impl Clone for Entity {
	fn clone(&self) -> Self {
		Entity {
			id: self.id,
			name: self.name.clone(),
			health: self.health,
			attack: self.attack,
			defense: self.defense,
		}
	}
}

impl std::fmt::Debug for Entity {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Entity {{ id: {}, name: {}, health: {}, attack: {}, defense: {} }}", 
			self.id, self.name, self.health, self.attack, self.defense)
	}
}

impl Entity {
	pub fn new(id: usize, name: String, health: i32, attack: u32, defense: i32) -> Self {
		Entity {
			id,
			name,
			health,
			attack,
			defense,
		}
	}

	pub fn get_id(&self) -> usize {
		self.id
	}
	
	pub fn get_name(&self) -> String {
		self.name.clone()
	}
	
	pub fn get_health(&self) -> i32 {
		self.health
	}
	
	pub fn get_attack(&self) -> u32 {
		self.attack
	}
	
	pub fn get_defense(&self) -> i32 {
		self.defense
	}

	pub fn set_health(&mut self, health: i32) {
		self.health = health;
	}
	
	pub fn set_attack(&mut self, attack: u32) {
		self.attack = attack;
	}
	
	pub fn set_defense(&mut self, defense: i32) {
		self.defense = defense;
	}
}