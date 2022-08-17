use std::env;
use std::fs;
use tokio;
use futures::StreamExt;
use telegram_bot::*;
use std::collections::HashMap;

use crate::Sotpal;
use crate::utils::{Error, Result};

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

	async fn add_player(&mut self, user: User) -> Result<()> {
		self.game.add_player(self.next_id, user.first_name)?;
		self.players.insert(user.id, self.next_id);
		self.next_id += 1;
		self.api.send(user.id.text("You were added to the game")).await;
		Ok(())
	}

	async fn remove_player(&mut self, user_id: UserId) -> Result<()> {
		self.game.remove_player(*self.players.get(&user_id).unwrap())?;
		self.players.remove(&user_id);
		self.api.send(user_id.text("See you soon")).await;
		Ok(())
	}

	async fn play_game(&mut self, user_id: UserId) -> Result<()> {
		self.game.ready()?;
		let topic = self.game.get_topic(*self.players.get(&user_id).unwrap()).unwrap();
		for (id, _) in &self.players {
			self.api.send(id.text(&topic)).await;
		}
		Ok(())
	}

	async fn add_topic(&mut self, user_id: UserId, topic: String) -> Result<()> {
		self.game.add_topic(
			*self.players.get(&user_id).unwrap(),
			topic,
		)?;
		self.api.send(user_id.text("Your topic was successfully added to the game")).await;
		Ok(())
	}

	async fn receive_message(&mut self, message: Message) -> Result<()> {
		if let MessageKind::Text { ref data, .. } = message.kind {
			if message.from.id == UserId::new(383471334) {
				match data.as_str() {
					"/print" => {
						self.api.send(message.from.id.text(&self.game.print_players())).await;
					},
					"/reset" => {
						self.players = HashMap::new();
						self.game = Sotpal::new();
						self.api.send(message.from.id.text("The game has been reset")).await;
					},
					"/test" => {
						let mut markup_message = message.from.id.text("Test markup message");

						let button = InlineKeyboardButton::callback("test text", "tcb");
						let mut row: Vec<InlineKeyboardButton> = Vec::new();
						row.push(button);
						let mut keyboard = InlineKeyboardMarkup::new();
						keyboard.add_row(row);

						markup_message.reply_markup(keyboard);
						self.api.send(markup_message).await;
					}
					_ => (),
				}
			}

			match (self.players.get(&message.from.id), data.as_str()) {
				(Some(_), "/start") => {
					self.api.send(message.from.id.text("You're alrerady playing")).await;
				},
				(None, "/start") => {
					self.add_player(message.from).await;
				},
				(Some(_), "/quit") => {
					self.remove_player(message.from.id).await;
				},
				(Some(_), "/play") => {
					self.play_game(message.from.id).await;
				},
				(Some(_), _) => {
					self.add_topic(message.from.id, data.clone()).await;
				},
				(None, _) => {
					self.api.send(message.from.id.text("You're not playing")).await;
				}
			}
		}
		Ok(())
	}

	async fn receive_callback(&self, cb: CallbackQuery) -> Result<()> {
		self.api.send(cb.from.text("Gotcha")).await;
		Ok(())
	}

	#[tokio::main]
	pub async fn start(mut self) {
		let mut stream = self.api.stream();
		while let Some(update) = stream.next().await {
			let update = update.unwrap();
			let (userid, result) = match update.kind {
				UpdateKind::Message(message) => (message.from.id, self.receive_message(message).await),
				UpdateKind::CallbackQuery(cb) => (cb.from.id, self.receive_callback(cb).await),
				_ => {
					(UserId::new(383471334), Err(Error::General("Unknown message kind received".to_string())))
				},
			};
			match result {
				Err(e) => {
					self.api.send(userid.text(format!("{}", e))).await;
				},
				_ => (),
			};
		}
	}
}

pub fn run() {
	let token = fs::read_to_string("sotpal_token")
		.expect("Could not read from file");

	let bot = TelegramBot::new(token);
	bot.start();
}