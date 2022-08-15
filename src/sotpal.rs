mod player;

use indexmap::IndexMap;
use rand::Rng;
use player::Player;

pub struct Sotpal {
	players: IndexMap<i32, Player>,
}

impl Sotpal {
	pub fn new() -> Self {
		Self {
			players: IndexMap::new(),
		}
	}

	pub fn is_a_player(&self, id: i32) -> bool {
		match self.players.get_index_of(&id) {
			Some(_) => true,
			None => false,
		}
	}

	pub fn add_player(&mut self, id:i32, name: String) -> bool {
		if name == "".to_string() || self.players.get_index_of(&id).is_some() {
			return false;
		}

		let player = Player::new (
			name,
		);

		self.players.insert(id, player);

		true
	}

	pub fn remove_player(&mut self, id: i32) -> bool {
		if self.players.get_index_of(&id).is_none() {
			return false;
		}

		self.players.remove(&id);

		true
	}

	pub fn is_playable(&mut self) -> bool {
		if self.players.len() < 3 {
			return false;
		}
		for (id, player) in &self.players {
			if !player.is_ready() {
				return false;
			}
		}
		true
	}

	pub fn add_topic(&mut self, player_id: i32, topic: String) -> bool {
		match self.players.get_mut(&player_id) {
			Some(player) => player.add_topic(topic),
			None => return false,
		};
		true
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

	pub fn print_players(&self) -> String {
		let mut string = "".to_string();
		for (_, player) in &self.players {
			string.push_str(&player.print());
			string.push_str("\n");
		}
		string
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
		let player1_id = 0;
		game.add_player(player1_id, "TestName".to_string());
		game.add_player(player1_id, "TestName2".to_string());
		assert_eq!(1, game.players.len());

		let test_topic = "test topic".to_string();
		game.add_topic(player1_id, test_topic.clone());
		game.add_topic(player1_id+1, test_topic);
	}

	#[test]
	fn test_get_topic() {
		let mut game = Sotpal::new();
		let player1_id = 0;
		game.add_player(player1_id, "TestName".to_string());
		let test_topic = "test topic".to_string();
		game.add_topic(player1_id, test_topic.clone());
		assert_eq!(game.get_topic(-1), "".to_string());
		assert_eq!(game.get_topic(player1_id), "".to_string());

		let player2_id = 1;
		game.add_player(player2_id, "TestName 2".to_string());
		let test_topic2 = "test topic 2".to_string();
		game.add_topic(player2_id, test_topic2.clone());
		assert_eq!(game.get_topic(-1), "".to_string());
		assert_eq!(game.get_topic(player2_id), test_topic);
		
		game.add_topic(player2_id, "test topic 2".to_string());
		assert_eq!(game.get_topic(player1_id), test_topic2);
	}
}