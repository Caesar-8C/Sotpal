use rand::Rng;

use crate::utils::{Error, Result};

pub struct Player {
	pub name: String,
	pub points: i32,
	pub topics: Vec<String>,
}

impl Player {
	pub fn new(name: String) -> Self {
		Self {
			name,
			points: 0,
			topics: Vec::new(),
		}
	}

	pub fn add_topic(&mut self, topic: String) -> Result<()> {
		if topic != "".to_string() {
			self.topics.push(topic);
			Ok(())
		}
		else {
			Err(Error::General("Tried to add empty topic".to_string()))
		}
	}

	pub fn draw_topic(&mut self) -> Result<String> {
		if self.topics.len() == 0 {
			Err(Error::General("No topics to get".to_string()))
		}
		else {
			let index = rand::thread_rng().gen_range(0..self.topics.len());
			Ok(self.topics.remove(index))
		}
	}

	pub fn is_ready(&self) -> bool {
		self.topics.len() >= 1
	}

	pub fn print(&self) -> String {
		format!(
			"{} has {} points and {} topics",
			self.name,
			self.points,
			self.topics.len()
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add_topic() {
		let player_name = "Test Name".to_string();
		let mut player = Player::new(player_name.clone());
		assert_eq!(player.name, player_name);
		assert_eq!(player.points, 0);
		assert_eq!(player.topics.len(), 0);

		let topic = "test topic".to_string();
		assert!(player.add_topic(topic.clone()).is_ok());
		assert_eq!(player.topics.len(), 1);
		assert_eq!(player.topics[0], topic);
		assert!(player.add_topic("".to_string()).is_err());
		assert_eq!(player.topics.len(), 1);
	}

	#[test]
	fn test_draw_topic() {
		let player_name = "Test Name".to_string();
		let mut player = Player::new(player_name.clone());

		let topic = "test topic".to_string();
		assert!(player.add_topic(topic.clone()).is_ok());
		if let Ok(t) = player.draw_topic() {
			assert_eq!(t, topic);
		};

		assert!(player.add_topic("test topic".to_string()).is_ok());
		assert!(player.add_topic("test topic 2".to_string()).is_ok());
		assert!(player.add_topic("test topic 3".to_string()).is_ok());
		assert!(player.add_topic("test topic 4".to_string()).is_ok());
		
		assert_eq!(player.topics.len(), 4);
		assert!(player.draw_topic().is_ok());
		assert_eq!(player.topics.len(), 3);
		assert!(player.draw_topic().is_ok());
		assert_eq!(player.topics.len(), 2);
		assert!(player.draw_topic().is_ok());
		assert_eq!(player.topics.len(), 1);
		assert!(player.draw_topic().is_ok());
		assert_eq!(player.topics.len(), 0);
		assert!(player.draw_topic().is_err());
	}
}