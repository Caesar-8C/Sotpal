use telegram_bot::*;

#[derive(Clone)]
pub struct Player {
    pub id: i32,
	pub game_id: i32,
	pub state: PlayerState,
}

impl Player{
	pub fn new(id: i32) -> Self {
		Self {
			id,
			game_id: 0,
			state: PlayerState::Greeting,
		}
	}
}

#[derive(Clone)]
pub enum PlayerState {
	Greeting,
	Local(Vec<String>),
	Playing,
	Joining,
	Guessing,
}

pub mod keyboards {
	use telegram_bot::*;
	use crate::Sotpal;

	pub fn greeting() -> InlineKeyboardMarkup {
		reply_markup!(inline_keyboard,
			["Rules" callback "1", "Create" callback "3"],
			["Local" callback "2", "Join"	callback "4"]
		)
	}

	pub fn local() -> InlineKeyboardMarkup {
		reply_markup!(inline_keyboard,
			["Draw Topic" callback "5", "Leave" callback "6"]
		)
	}

	pub fn playing() -> InlineKeyboardMarkup {
		reply_markup!(inline_keyboard,
			["Add Topic" callback "7", "Start guessing" callback "8"]
		)
	}

	pub fn guessing(game: &Sotpal) -> InlineKeyboardMarkup {
		let mut row: Vec<InlineKeyboardButton>;
		let mut button: InlineKeyboardButton;
		let mut keyboard = InlineKeyboardMarkup::new();

		for (_, player) in &game.players {
			row = Vec::new();
			button  = InlineKeyboardButton::callback(&player.name, &player.name);
			row.push(button);
			keyboard.add_row(row);
		}

		keyboard
	}
}

pub mod replies {
	use crate::Sotpal;

	pub fn greeting() -> String {
		"Hello new player!\nWelcome!".to_string()
	}

	pub fn local(topics_num: usize) -> String {
		let mut reply = "This is a local game\n\n".to_string();
		reply.push_str(&format!("You have {} topics\n", topics_num));
		reply
	}

	pub fn playing(game: &Sotpal) -> String {
		let mut reply = "The game is on!\n\nPoints : Names : Topics\n".to_string();
		for (_, player) in &game.players {
			reply.push_str(&format!("{} : {} : {}\n", player.points, player.name, player.topics.len()))
		}
		reply
	}
}