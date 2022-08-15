use rand::Rng;

pub struct Player {
	name: String,
	points: i32,
	topics: Vec<String>,
}

impl Player {
	pub fn new(name: String) -> Self {
		Self {
			name,
			points: 0,
			topics: Vec::new(),
		}
	}

	pub fn add_topic(&mut self, topic: String) -> bool {
		if topic != "".to_string() {
			self.topics.push(topic);
			true
		}
		else {
			false
		}
	}

	pub fn get_topic(&mut self) -> String {
		if self.topics.len() == 0 {
			"".to_string()
		}
		else {
			let index = rand::thread_rng().gen_range(0..self.topics.len());
			self.topics.remove(index)
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

	pub fn list_topics(&self) {
		println!("{:?}", self.topics);
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
		player.add_topic(topic.clone());
		assert_eq!(player.topics.len(), 1);
		assert_eq!(player.topics[0], topic);
		player.add_topic("".to_string());
		assert_eq!(player.topics.len(), 1);
	}

	#[test]
	fn test_get_topic() {
		let player_name = "Test Name".to_string();
		let mut player = Player::new(player_name.clone());

		let topic = "test topic".to_string();
		player.add_topic(topic.clone());
		assert_eq!(player.get_topic(), topic);

		player.add_topic("test topic".to_string());
		player.add_topic("test topic 2".to_string());
		player.add_topic("test topic 3".to_string());
		player.add_topic("test topic 4".to_string());
		
		assert_eq!(player.topics.len(), 4);
		player.get_topic();
		assert_eq!(player.topics.len(), 3);
		player.get_topic();
		assert_eq!(player.topics.len(), 2);
		player.get_topic();
		assert_eq!(player.topics.len(), 1);
		player.get_topic();
		assert_eq!(player.topics.len(), 0);
		assert_eq!(player.get_topic(), "".to_string());
	}
}