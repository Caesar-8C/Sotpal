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
	ThisGuessing,
	OtherGuessing,
}

pub mod keyboards {
	use telegram_bot::*;
	use crate::Sotpal;

	pub fn greeting() -> InlineKeyboardMarkup {
		reply_markup!(inline_keyboard,
			["Example" url "https://www.youtube.com/watch?v=3UAOs9B9UH8&list=PLfx61sxf1Yz2I-c7eMRk9wBUUDCJkU7H0&index=1", "Create Game" callback "create"],
			["Local Game" callback "local", "Join Game" callback "join"]
		)
	}

	pub fn local() -> InlineKeyboardMarkup {
		reply_markup!(inline_keyboard,
			["Random Wiki Page" url "https://en.wikipedia.org/wiki/Special:Random", "Draw Topic" callback "local draw"],
			["Leave" callback "local leave"]
		)
	}

	pub fn playing() -> InlineKeyboardMarkup {
		reply_markup!(inline_keyboard,
			["Random Wiki Page" url "https://en.wikipedia.org/wiki/Special:Random", "Guess" callback "guess"],
			["Leave" callback "playing leave"]
		)
	}

	pub fn guessing(game: &Sotpal) -> InlineKeyboardMarkup {
		let mut row: Vec<InlineKeyboardButton>;
		let mut button: InlineKeyboardButton;
		let mut keyboard = InlineKeyboardMarkup::new();

		for (id, player) in &game.players {
			row = Vec::new();
			button  = InlineKeyboardButton::callback(&player.name, &format!("{}", id));
			row.push(button);
			keyboard.add_row(row);
		}

		keyboard
	}
}

pub mod replies {
	use crate::Sotpal;

	pub fn greeting() -> String {
		let mut reply = "Hello and welcome to\nSome of These People are Lying!\n\n".to_string();
		reply.push_str("The rules are simple: one player creates the game ");
		reply.push_str("and sends the code for the others to join. ");
		reply.push_str("Everyone goes to a random wiki page, reads it, ");
		reply.push_str("and sends the page title to me. Guesser then presses ");
		reply.push_str("the guess button. I choose the topic and send it to everyone. ");
		reply.push_str("The person who read the topic tells what it was about, ");
		reply.push_str("others lie and try to convince the guesser that it was ");
		reply.push_str("them, who read the wiki. Guesser then tries to guess, ");
		reply.push_str("who is telling the truth.\n\nHave fun!");
		reply
	}

	pub fn local(topics_num: usize) -> String {
		let mut reply = "This is a local game\n\n".to_string();
		reply.push_str(&format!("You have {} topics\n", topics_num));
		reply
	}

	pub fn local_draw(topic: String, topics_num: usize) -> String {
		format!("The chosen topic is:\n{}\n\nYou still have {} other topics.", topic, topics_num)
	}

	pub fn playing(game: &Sotpal, game_id: i32) -> String {
		let mut reply = format!("The game is on!\nHere's the game id: {}\n\nPoints : Names : Topics\n", game_id);
		for (_, player) in &game.players {
			reply.push_str(&format!("{} : {} : {}\n", player.points, player.name, player.topics.len()))
		}
		reply
	}

	pub fn joining() -> String {
		"Please send the game id to join\n(Send any number to go back)".to_string()
	}

	pub fn join_fail() -> String {
		"Joining failed, wrong game id?".to_string()
	}

	pub fn guessing(topic: String) -> String {
		let mut reply = "The game is on!\nTopic: ".to_string();
		reply.push_str(&topic);
		reply.push_str("\nGuess who's topic this is");
		reply
	}

	pub fn other_guessing(name: String, topic: String) -> String {
		format!("{} is guessing, who's lying about:\n{}", name, topic)
	}

	pub fn guessed_player() -> String {
		"Guesser believed you were saying the truth\nYou get a point!".to_string()
	}
}