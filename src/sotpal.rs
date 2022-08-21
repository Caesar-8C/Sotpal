pub mod player;

use indexmap::IndexMap;
use rand::Rng;

use player::Player;
use crate::utils::{Error, Result};

pub struct Sotpal {
	pub players: IndexMap<i32, Player>,
	pub topic: Result<String>,
}

impl Sotpal {
	pub fn new() -> Self {
		Self {
			players: IndexMap::new(),
			topic: Err(Error::General("Topic requested, but none set".to_string())),
		}
	}

	pub fn is_a_player(&self, id: i32) -> bool {
		self.players.get_index_of(&id).is_some()
	}

	pub fn add_player(&mut self, id:i32, name: String) -> Result<()> {
		if name == "".to_string() {
			Err(Error::General("Tried to create a player with empty name".to_string()))
		}
		else if self.players.get_index_of(&id).is_some() {
			Err(Error::General("Tried to create a player that already exist".to_string()))
		}
		else {
			let player = Player::new(name);
			self.players.insert(id, player);

			Ok(())
		}
	}

	pub fn remove_player(&mut self, id: i32) -> Result<()> {
		if self.players.get_index_of(&id).is_none() {
			Err(Error::General("Tried to remove a non existent player".to_string()))
		}
		else {
			self.players.remove(&id);
			Ok(())
		}
	}

	pub fn ready(&mut self) -> Result<()> {
		if self.players.len() < 3 {
			return Err(Error::General("Too few players".to_string()));
		}
		for (_, player) in &self.players {
			if !player.is_ready() {
				return Err(Error::General(format!("Player {} has 0 topics", player.name)));
			}
		}
		Ok(())
	}

	pub fn add_topic(&mut self, player_id: i32, topic: String) -> Result<()> {
		match self.players.get_mut(&player_id) {
			Some(player) => {
				player.add_topic(topic)
			},
			None => Err(Error::General("Player does not exist".to_string())),
		}
	}

	pub fn draw_topic(&mut self, guesser_id: i32) -> Result<String> {
		self.ready()?;

		let index = rand::thread_rng().gen_range(0..self.players.len());
		self.topic = match self.players.get_index_of(&guesser_id) {
			None => Err(Error::General("Unknown guesser".to_string())),
			Some(i) if i == index => self.draw_topic(guesser_id),
			_ => match self.players.get_index_mut(index) {
				None => Err(Error::General("Something's wrong with randomizer".to_string())),
				Some(player) => player.1.draw_topic(),
			},
		};
		self.topic.clone()
	}

	pub fn reset_topic(&mut self) {
		self.topic = Err(Error::General("Topic requested, but none set".to_string()));
	}

	pub fn print_players(&self) -> String {
		let mut string = "".to_string();
		for (_, player) in &self.players {
			string.push_str(&player.print());
			string.push_str("\n");
		}
		string
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add_player_topic() {
		let mut game = Sotpal::new();
		let player1_id = 0;
		assert!(game.add_player(player1_id, "TestName".to_string()).is_ok());
		assert!(game.add_player(player1_id, "TestName2".to_string()).is_err());
		assert_eq!(1, game.players.len());

		let test_topic = "test topic".to_string();
		assert!(game.add_topic(player1_id, test_topic.clone()).is_ok());
		assert!(game.add_topic(player1_id+1, test_topic).is_err());
	}

	#[test]
	fn test_draw_topic() {
		let mut game = Sotpal::new();
		let player1_id = 0;
		assert!(game.add_player(player1_id, "TestName".to_string()).is_ok());
		let test_topic = "test topic".to_string();
		assert!(game.add_topic(player1_id, test_topic.clone()).is_ok());
		assert!(game.draw_topic(-1).is_err());
		assert!(game.draw_topic(player1_id).is_err());

		let player2_id = 1;
		assert!(game.add_player(player2_id, "TestName 2".to_string()).is_ok());
		let test_topic2 = "test topic 2".to_string();
		assert!(game.add_topic(player2_id, test_topic2.clone()).is_ok());

		let player3_id = 3;
		assert!(game.add_player(player3_id, "TestName 3".to_string()).is_ok());
		let test_topic3 = "test topic 3".to_string();
		assert!(game.add_topic(player3_id, test_topic3.clone()).is_ok());

		assert!(game.ready().is_ok());
		assert!(game.draw_topic(-1).is_err());
		assert!(game.draw_topic(player2_id).is_ok());
		
		assert!(game.add_topic(player1_id, "test topic 1".to_string()).is_ok());
		assert!(game.add_topic(player3_id, "test topic 3".to_string()).is_ok());
		assert!(game.draw_topic(player1_id).is_ok());
	}
}