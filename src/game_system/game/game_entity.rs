pub struct Entity {
	pub id: u32,
	pub name: String,
	pub health: u32,
	pub attack: u32,
	pub defense: u32,
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
	pub fn new(id: u32, name: String, health: u32, attack: u32, defense: u32) -> Self {
		Entity {
			id,
			name,
			health,
			attack,
			defense,
		}
	}
	
	pub fn get_id(&self) -> u32 {
		self.id
	}
	
	pub fn get_name(&self) -> String {
		self.name.clone()
	}
	
	pub fn get_health(&self) -> u32 {
		self.health
	}
	
	pub fn get_attack(&self) -> u32 {
		self.attack
	}
	
	pub fn get_defense(&self) -> u32 {
		self.defense
	}
	
	pub fn set_health(&mut self, health: u32) {
		self.health = health;
	}
	
	pub fn set_attack(&mut self, attack: u32) {
		self.attack = attack;
	}
	
	pub fn set_defense(&mut self, defense: u32) {
		self.defense = defense;
	}
}