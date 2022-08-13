mod player;

use std::collections::HashMap;
use player::Player;

pub struct Sotpal {
	players: HashMap<i32, Player>,
	next_id: i32,
}

impl Sotpal {
	pub fn new() -> Self {
		Self {
			players: HashMap::new(),
			next_id: 0,
		}
	}

	pub fn add_player(&mut self, name: String) -> i32 {
		if name == "".to_string() {
			return -1;
		}

		let player_id = self.next_id;

		let player = Player::new (
			name,
		);

		self.players.insert(player_id, player);
		self.next_id += 1;

		player_id
	}

	pub fn add_topic(&mut self, player_id: i32, topic: String) {
		match self.players.get_mut(&player_id) {
			Some(player) => player.add_topic(topic),
			None => return,
		};
	}

	pub fn get_topic(&mut self, player_id: i32) -> String {
		"".to_string()
	}

	pub fn print_players(&self) {
		for (_, player) in &self.players {
			player.print();
		}
	}

	pub fn list_topics(&self) {
		for (_, player) in &self.players {
			player.list_topics();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add_player_topic() {
		let mut game = Sotpal::new();
		let test_player = game.add_player("TestName".to_string());
		assert_eq!(test_player, 0);
		assert_eq!(game.next_id, 1);

		let test_topic = "test topic".to_string();
		game.add_topic(test_player, test_topic.clone());
		game.add_topic(test_player+1, test_topic);
	}
}