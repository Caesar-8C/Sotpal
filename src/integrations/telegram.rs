use std::env;
use std::fs;
use tokio;
use futures::StreamExt;
use telegram_bot::*;
use std::collections::HashMap;

use crate::Sotpal;

struct TelegramBot {
	game: Sotpal,
	players: HashMap<UserId, i32>,
	next_id: i32,
	api: Api,
}

impl TelegramBot {
	pub fn new(token: String) -> Self {
		Self {
			game: Sotpal::new(),
			players: HashMap::new(),
			next_id: 0,
			api: Api::new(token),
		}
	}

	async fn add_player(&mut self, user: User) {
		if self.game.add_player(self.next_id, user.first_name) {
			self.players.insert(user.id, self.next_id);
			self.next_id += 1;
			self.api.send(user.id.text("You were added to the game")).await;
		}
		else {
			self.api.send(user.id.text("Sorry, something broke")).await;
		}
	}

	async fn remove_player(&mut self, user_id: UserId) {
		if self.game.remove_player(*self.players.get(&user_id).unwrap()) {
			self.players.remove(&user_id);
			self.api.send(user_id.text("See you soon")).await;
		}
		else {
			self.api.send(user_id.text("Sorry, something broke")).await;
		}
	}

	async fn play_game(&mut self, user_id: UserId) {
		if self.game.is_playable() {
			let topic = self.game.get_topic(*self.players.get(&user_id).unwrap());

			for (id, _) in &self.players {
				self.api.send(id.text(&topic)).await;
			}
		}
		else {
			self.api.send(user_id.text("Game is not ready yet")).await;
		}
	}

	async fn add_topic(&mut self, user_id: UserId, topic: String) {
		if self.game.add_topic(
			*self.players.get(&user_id).unwrap(),
			topic,
		) {
			self.api.send(user_id.text("Your topic was successfully added to the game")).await;
		}
		else {
			self.api.send(user_id.text("Sorry, something broke")).await;
		}
	}

	#[tokio::main]
	pub async fn start(mut self) {
		let mut stream = self.api.stream();
		while let Some(update) = stream.next().await {
			let update = update.unwrap();
			if let UpdateKind::Message(message) = update.kind {
				if let MessageKind::Text { ref data, .. } = message.kind {
					match data.as_str() {
						"/start" => match self.players.get(&message.from.id) {
							Some(_) => {
								self.api.send(message.from.id.text("You're alrerady playing")).await;
							},
							None => {
								self.add_player(message.from).await;
							},
						},
						"/quit" => match self.players.get(&message.from.id) {
							Some(_) => {
								self.remove_player(message.from.id).await;
							}
							None => {
								self.api.send(message.from.id.text("You're not playing")).await;
							}
						},
						"/play" => match self.players.get(&message.from.id) {
							Some(_) => {
								self.play_game(message.from.id).await;
							}
							None => {
								self.api.send(message.from.id.text("You're not playing")).await;
							}
						},
						"/print" => { 
							if message.from.id == UserId::new(383471334) {
								self.api.send(message.from.id.text(&self.game.print_players())).await;
							}
							else {
								self.api.send(message.from.id.text(
									"Seems that someone lacks authorisation"
								)).await;
								let last = message.from.last_name.unwrap().as_str();
								self.api.send(UserId::new(383471334).text(
									message.from.first_name
								)).await;
							}
						},
						"/reset" => { 
							if message.from.id == UserId::new(383471334) {
								self.players = HashMap::new();
								self.game = Sotpal::new();
								self.api.send(message.from.id.text("The game has been reset")).await;
							}
							else {
								self.api.send(message.from.id.text(
									"Seems that someone lacks authorisation"
								)).await;
								let last = message.from.last_name.unwrap().as_str();
								self.api.send(UserId::new(383471334).text(
									message.from.first_name
								)).await;
							}
						},
						_ => match self.players.get(&message.from.id) {
							Some(_) => {
								self.add_topic(message.from.id, data.clone()).await;
							}
							None => {
								self.api.send(message.from.id.text("You're not playing")).await;
							}
						},
					}
				}
			}
		}
	}
}

pub fn run() {
	let token = fs::read_to_string("sotpal_token")
		.expect("Could not read from file");

	let bot = TelegramBot::new(token);
	bot.start();
}