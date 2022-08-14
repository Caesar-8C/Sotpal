mod player;

use indexmap::IndexMap;
use rand::Rng;
use player::Player;

pub struct Sotpal {
	players: IndexMap<i32, Player>,
	next_id: i32,
}

impl Sotpal {
	pub fn new() -> Self {
		Self {
			players: IndexMap::new(),
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

	pub fn get_topic(&mut self, guesser_id: i32) -> String {
		if self.players.len() < 2 {
			return "".to_string();
		}

		let index = rand::thread_rng().gen_range(0..self.players.len());
		match self.players.get_index_of(&guesser_id) {
			None => "".to_string(),
			Some(i) if i == index => self.get_topic(guesser_id),
			_ => self.players.get_index_mut(index).unwrap().1.get_topic(),
		}
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

	#[test]
	fn test_get_topic() {
		let mut game = Sotpal::new();
		let test_player = game.add_player("TestName".to_string());
		let test_topic = "test topic".to_string();
		game.add_topic(test_player, test_topic.clone());
		assert_eq!(game.get_topic(-1), "".to_string());
		assert_eq!(game.get_topic(test_player), "".to_string());

		let test_player2 = game.add_player("TestName 2".to_string());
		let test_topic2 = "test topic 2".to_string();
		game.add_topic(test_player2, test_topic2.clone());
		assert_eq!(game.get_topic(-1), "".to_string());
		assert_eq!(game.get_topic(test_player2), test_topic);
		
		game.add_topic(test_player2, "test topic 2".to_string());
		assert_eq!(game.get_topic(test_player), test_topic2);
	}
}