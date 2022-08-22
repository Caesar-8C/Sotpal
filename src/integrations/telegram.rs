mod utils;

use std::env;
use std::fs;
use tokio;
use futures::StreamExt;
use telegram_bot::*;
use std::collections::HashMap;
use rand::Rng;

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

	async fn add_local_topic(&mut self, user: User, chat: MessageChat, message_id: MessageId, data: String, topics: Vec<String>) {
		self.api.send(DeleteMessage::new(chat, message_id)).await;

		let mut new_topics = topics;
		new_topics.push(data);
		self.players.get_mut(&user.id).unwrap().state = PlayerState::Local(new_topics.clone());

		let mut reply = user.text(replies::local(new_topics.len()));
		reply.reply_markup(keyboards::local());
		self.api.send(reply).await;
	}

	async fn draw_local_topic(&mut self, user: User, topics: Vec<String>) {
		let mut new_topics = topics.clone();
		let index = rand::thread_rng().gen_range(0..topics.len());
		let topic = new_topics.remove(index);
		self.players.get_mut(&user.id).unwrap().state = PlayerState::Local(new_topics.clone());

		let mut reply = user.text(replies::local_draw(topic, new_topics.len()));
		reply.reply_markup(keyboards::local());
		self.api.send(reply).await;
	}

	async fn to_greeting_state(&mut self, user: User) {
		self.players.get_mut(&user.id).unwrap().state = PlayerState::Greeting;

		let mut reply = user.text(replies::greeting());
		reply.reply_markup(keyboards::greeting());
		self.api.send(reply).await;
	}

	async fn join_game(&mut self, user: User, game_id: i32) -> Result<()> {
		let mut player = self.players.get_mut(&user.id).unwrap();
		let mut game = self.games.get_mut(&game_id).unwrap();
		game.add_player(player.id, user.first_name.clone())?;
		player.game_id = game_id;
		player.state = PlayerState::Playing;

		let mut reply = user.text(replies::playing(game, game_id));
		reply.reply_markup(keyboards::playing());
		self.api.send(reply).await;
		Ok(())
	}

	async fn fail_join_game(&mut self, user: User) {
		self.api.send(user.text(replies::join_fail())).await;
		let mut player = self.players.get_mut(&user.id).unwrap();
		player.state = PlayerState::Greeting;

		let mut reply = user.text(replies::greeting());
		reply.reply_markup(keyboards::greeting());
		self.api.send(reply).await;
	}

	async fn try_join_game(&mut self, user: User, data: String) -> Result<()> {
		let key = data.parse::<i32>()?;
		match self.games.get(&key) {
			Some(_) => self.join_game(user, key).await?,
			None => self.fail_join_game(user).await,
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

		let mut reply = user.text(replies::playing(game, game_id));
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

	async fn message_on_greeting(&self, user: User) {
		let mut reply = user.text(replies::greeting());
		reply.reply_markup(keyboards::greeting());
		self.api.send(reply).await;
	}

	async fn message_on_this_guessing(&self, user: User) -> Result<()> {
		let game_id = self.players.get(&user.id).unwrap().game_id;
		let game = self.games.get(&game_id).unwrap();

		let mut reply = user.text(replies::guessing(game.topic.clone()?));
		reply.reply_markup(keyboards::guessing(game));
		self.api.send(reply).await;

		Ok(())
	}

	async fn on_other_guessing(&self, user: User) -> Result<()> {
		let player = self.players.get(&user.id).unwrap();
		let game = self.games.get(&player.game_id).unwrap();
		let guesser_id = game.guesser.clone()?;
		let guesser = game.players.get(&guesser_id).unwrap();
		let guesser_name = guesser.name.clone();
		let reply = user.text(replies::other_guessing(guesser_name, game.topic.clone()?));
		self.api.send(reply).await;
		Ok(())
	}

	async fn receive_message_result(&mut self, message: Message) -> Result<()> {
		match message.kind {
			MessageKind::Text { ref data, .. } => {
				match self.get_player_state(message.from.id) {
					None => self.add_player(message.from).await,
					Some(PlayerState::Local(topics)) => self.add_local_topic(message.from, message.chat, message.id, data.clone(), topics).await,
					Some(PlayerState::Playing) => self.add_topic(message.from, data.clone()).await?,
					Some(PlayerState::Joining) => self.try_join_game(message.from, data.clone()).await?,
					Some(PlayerState::Greeting) => self.message_on_greeting(message.from).await,
					Some(PlayerState::ThisGuessing) => self.message_on_this_guessing(message.from).await?,
					Some(PlayerState::OtherGuessing) => self.on_other_guessing(message.from).await?,
				};
				Ok(())
			},
			_ => Err(Error::General("Unknown message kind received".to_string())),
		}
	}

	async fn receive_message(&mut self, message: Message) {
		match self.receive_message_result(message.clone()).await {
			Ok(_) => (),
			Err(e) => {
				self.api.send(message.from.text(&format!("{}", e))).await;
			},
		};
	}

	fn unexpected_callback(&self) -> Result<()> {
		Err(Error::General("Unexpected callback received".to_string()))
	}

	async fn local_callback(&mut self, user:User, data: String, topics: Vec<String>) -> Result<()> {
		match data.as_str() {
			"local draw" => self.draw_local_topic(user, topics).await,
			"local leave" => self.to_greeting_state(user).await,
			_ => self.unexpected_callback()?,
		}

		Ok(())
	}

	async fn create_local_game(&mut self, user: User) {
		let topics: Vec<String> = Vec::new();
		self.players.get_mut(&user.id).unwrap().state = PlayerState::Local(topics.clone());

		let mut reply = user.text(replies::local(topics.len()));
		reply.reply_markup(keyboards::local());
		self.api.send(reply).await;
	}

	async fn create_game(&mut self, user: User) -> Result<()> {
		let mut game = Sotpal::new();
		let game_id = rand::thread_rng().gen_range(10000000..99999999);
		// TODO exclude existing ids

		let mut player = self.players.get_mut(&user.id).unwrap();
		game.add_player(player.id, user.first_name.clone())?;
		player.game_id = game_id;

		let mut reply = user.text(replies::playing(&game, game_id));
		self.games.insert(game_id, game);
		reply.reply_markup(keyboards::playing());
		self.api.send(reply).await;

		Ok(())
	}

	async fn enter_join_state(&mut self, user: User) {
		self.players.get_mut(&user.id).unwrap().state = PlayerState::Joining;

		let reply = user.text(replies::joining());
		self.api.send(reply).await;
	}

	async fn greeting_callback(&mut self, user:User, data: String) -> Result<()> {
		match data.as_str() {
			"local" => self.create_local_game(user).await,
			"create" => self.create_game(user).await?,
			"join" => self.enter_join_state(user).await,
			_ => self.unexpected_callback()?,
		}

		Ok(())
	}

	async fn playing_leave(&mut self, user: User) -> Result<()> {
		let player = self.players.get(&user.id).unwrap();
		let player_id = player.id;
		let game_id = player.game_id;
		let mut game = self.games.get_mut(&game_id).unwrap();
		game.remove_player(player_id)?;
		self.to_greeting_state(user).await;
		Ok(())
	}

	async fn guess(&mut self, user: User) -> Result<()> {
		let guesser = self.players.get(&user.id).unwrap();
		let guesser_id = guesser.id;
		let game_id = guesser.game_id;
		let mut game = self.games.get_mut(&game_id).unwrap();

		let topic = game.draw_topic(guesser_id)?;

		for (user_id, player) in self.players.iter_mut() {
			if player.game_id == game_id {
				if player.id == guesser_id {
					player.state = PlayerState::ThisGuessing;
					let mut reply = user.text(replies::guessing(topic.clone()));
					reply.reply_markup(keyboards::guessing(game));
					self.api.send(reply).await;
				}
				else {
					player.state = PlayerState::OtherGuessing;
					let reply = user_id.text(replies::other_guessing(user.first_name.clone(), topic.clone()));
					self.api.send(reply).await;
				}
			}
		}

		Ok(())
	}

	async fn playing_callback(&mut self, user:User, data: String) -> Result<()> {
		match data.as_str() {
			"guess" => self.guess(user).await?,
			"playing leave" => self.playing_leave(user).await?,
			_ => self.unexpected_callback()?,
		}

		Ok(())
	}

	async fn this_guessing_callback(&mut self, user:User, data: String) -> Result<()> {
		let guess = data.parse::<i32>().unwrap();
		let guesser = self.players.get(&user.id).unwrap();
		let guesser_id = guesser.id;
		let game_id = guesser.game_id;
		let mut game = self.games.get_mut(&game_id).unwrap();

		game.give_point(guess);
		if guess == game.reader.clone()? {
			game.give_point(guesser_id);
		}

		for (user_id, player) in self.players.iter_mut() {
			if player.game_id == game_id {
				player.state = PlayerState::Playing;

				let mut reply = user_id.text(replies::playing(game, game_id));
				reply.reply_markup(keyboards::playing());
				self.api.send(reply).await;
			}
		}

		Ok(())
	}

	async fn receive_callback_result(&mut self, cb: CallbackQuery) -> Result<()> {
		match cb.data {
			Some(data) => {
				match self.get_player_state(cb.from.id) {
					None |
					Some(PlayerState::Joining) => self.unexpected_callback()?,
					Some(PlayerState::Local(topics)) => self.local_callback(cb.from, data.clone(), topics).await?,
					Some(PlayerState::Greeting) => self.greeting_callback(cb.from, data.clone()).await?,
					Some(PlayerState::Playing) => self.playing_callback(cb.from, data.clone()).await?,
					Some(PlayerState::ThisGuessing) => self.this_guessing_callback(cb.from, data.clone()).await?,
					Some(PlayerState::OtherGuessing) => self.on_other_guessing(cb.from).await?,
				};
				Ok(())
			},
			None => Err(Error::General("Empty callback received".to_string())),
		}
	}

	async fn receive_callback(&mut self, cb: CallbackQuery) {
		match self.receive_callback_result(cb.clone()).await {
			Ok(_) => (),
			Err(e) => {
				self.api.send(cb.from.text(&format!("{}", e))).await;
			},
		};
	}

	#[tokio::main]
	pub async fn start(mut self) {
		let mut stream = self.api.stream();
		while let Some(update) = stream.next().await {
			if let Ok(update) = update {
				match update.kind {
					UpdateKind::Message(message) => self.receive_message(message).await,
					UpdateKind::CallbackQuery(cb) => self.receive_callback(cb).await,
					_ => {
						self.api.send(self.master.text("Unknown update kind received")).await;
					},
				};
			}
			else {
				self.api.send(self.master.text("Receiving update failed")).await;
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