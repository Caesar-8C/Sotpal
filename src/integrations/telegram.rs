mod utils;

use std::env;
use std::fs;
use tokio;
use futures::StreamExt;
use telegram_bot::*;
use std::collections::HashMap;

use crate::Sotpal;
use crate::utils::{Error, Result};
use utils::{Player, PlayerState, keyboards, replies};

struct TelegramBot {
	players: HashMap<UserId, Player>,
	games: HashMap<i32, Sotpal>,
	next_id: i32,
	api: Api,
	master: UserId,
}

impl TelegramBot {
	pub fn new(token: String, master_id: i64) -> Self {
		Self {
			players: HashMap::new(),
			games: HashMap::new(),
			next_id: 0,
			api: Api::new(token),
			master: UserId::new(master_id),
		}
	}

	async fn add_player(&mut self, user: User) {
		let player = Player::new(self.next_id);
		self.next_id += 1;
		self.players.insert(user.id, player);

		let mut reply = user.text(replies::greeting());
		reply.reply_markup(keyboards::greeting());
		self.api.send(reply).await;
	}

	async fn add_local_topic(&mut self, user: User, data: String, topics: Vec<String>) {
		let mut new_topics = topics;
		new_topics.push(data);
		self.players.get_mut(&user.id).unwrap().state = PlayerState::Local(new_topics.clone());

		let mut reply = user.text(replies::local(new_topics.len()));
		reply.reply_markup(keyboards::local());
		self.api.send(reply).await;
	}

	async fn join_game(&mut self, user: User, game_id: i32) -> Result<()> {
		let mut player = self.players.get_mut(&user.id).unwrap();
		let mut game = self.games.get_mut(&game_id).unwrap();
		game.add_player(player.id, user.first_name.clone())?;
		player.game_id = game_id;
		player.state = PlayerState::Playing;

		let mut reply = user.text(replies::playing(game));
		reply.reply_markup(keyboards::playing());
		self.api.send(reply).await;
		Ok(())
	}

	async fn try_join_game(&mut self, user: User, data: String) -> Result<()> {
		let key = data.parse::<i32>()?;
		match self.games.get(&key) {
			Some(game) => {
				self.join_game(user, key);
			}
			None => {
				// get error
				// return to greeting
			}
		};
		Ok(())
	}

	// async fn remove_player(&mut self, user_id: UserId) -> Result<()> {
	// 	self.game.remove_player(*self.players.get(&user_id))?;
	// 	self.players.remove(&user_id);
	// 	self.api.send(user_id.text("See you soon")).await;
	// 	Ok(())
	// }
	//
	// async fn play_game(&mut self, user_id: UserId) -> Result<()> {
	// 	self.game.ready()?;
	// 	let topic = self.game.get_topic(*self.players.get(&user_id).unwrap()).unwrap();
	// 	for (id, _) in &self.players {
	// 		self.api.send(id.text(&topic)).await;
	// 	}
	// 	Ok(())
	// }

	async fn add_topic(&mut self, user: User, topic: String) -> Result<()> {
		let game_id = self.players.get(&user.id).unwrap().game_id;
		let mut game = self.games.get_mut(&game_id).unwrap();
		game.add_topic(
			self.players.get(&user.id).unwrap().id,
			topic,
		)?;

		let mut reply = user.text(replies::playing(game));
		reply.reply_markup(keyboards::playing());
		self.api.send(reply).await;
		Ok(())
	}

	fn get_player_state(&self, id: UserId) -> Option<PlayerState> {
		if self.players.contains_key(&id) {
			Some(self.players.get(&id).unwrap().state.clone())
		}
		else {
			None
		}
	}

	async fn receive_message_result(&mut self, message: Message) -> Result<()> {
		match message.kind {
			MessageKind::Text { ref data, .. } => {
				match self.get_player_state(message.from.id) {
					None => self.add_player(message.from).await,
					Some(PlayerState::Local(topics)) => self.add_local_topic(message.from, data.clone(), topics).await,
					Some(PlayerState::Playing) => self.add_topic(message.from, data.clone()).await?,
					Some(PlayerState::Joining) => self.try_join_game(message.from, data.clone()).await?,
					Some(PlayerState::Greeting) => (),
					Some(PlayerState::Guessing) => (),
				};
				// let mut msg = message.from.text("Received message");
				// msg.reply_markup(keyboards::greeting());
				// self.api.send(msg).await;
			}
			_ => {
				self.api.send(message.from.text("Unknown message kind received")).await;
			}
		};
		Ok(())
	}

	async fn receive_message(&mut self, message: Message) {
		match self.receive_message_result(message).await {
			Ok(_) => (),
			Err(e) => (),
		};
	}

	async fn receive_callback_result(&self, cb: CallbackQuery) -> Result<()> {
		match cb.data {
			Some(data) => {
				self.api.send(cb.from.text(data)).await;
			},
			None => (),
		};
		Ok(())
	}

	async fn receive_callback(&mut self, cb: CallbackQuery) {
		match self.receive_callback_result(cb).await {
			Ok(_) => (),
			Err(e) => (),
		};
	}

	#[tokio::main]
	pub async fn start(mut self) {
		let mut stream = self.api.stream();
		while let Some(update) = stream.next().await {
			let update = update.unwrap();
			match update.kind {
				UpdateKind::Message(message) => self.receive_message(message).await,
				UpdateKind::CallbackQuery(cb) => self.receive_callback(cb).await,
				_ => {
					self.api.send(self.master.text("Unknown message kind received")).await;
				},
			}
		}
	}
}

pub fn run() {
	let master_id = fs::read_to_string("sotpal_master")
		.expect("Could not read from file").parse::<i64>().unwrap();
	let token = fs::read_to_string("sotpal_token")
		.expect("Could not read from file");

	let bot = TelegramBot::new(token, master_id);
	bot.start();
}