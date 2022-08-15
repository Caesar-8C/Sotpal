use std::io;
use std::process;

use crate::Sotpal;

fn get_input() -> String {
	let mut input = String::new();
	io::stdin().read_line(&mut input);
	input.trim_end().to_string()
}

fn main_menu() {
	println!("1> add a player");
	println!("2> add a topic");
	println!("3> start the game");
	println!("4> exit");
}

fn add_player_menu(game: &mut Sotpal, id: i32) {
	println!("Input name");
	let name = get_input();
	game.add_player(id, name.clone());
	println!("Added player {name} with id {id}")
 }

fn add_topic_menu(game: &mut Sotpal) {
	println!("Input player id");
	let id = get_input();
	println!("Input topic");
	let topic = get_input();

	game.add_topic(id.parse::<i32>().unwrap(), topic);
	print!("{}[2J", 27 as char);
}

fn start_the_game(game: &mut Sotpal) {
	println!("Who's gonna guess? (input player id)");
	let id = get_input();
	println!("{}", game.get_topic(id.parse::<i32>().unwrap()));
}

pub fn run() {
	let mut game = Sotpal::new();
	let mut next_id: i32 = 0;
	loop {
		main_menu();
		let input = get_input();

		match input.as_str() {
			"1" => { add_player_menu(&mut game, next_id); next_id += 1; },
			"2" => add_topic_menu(&mut game),
			"3" => start_the_game(&mut game),
			"4" => process::exit(0),
			_ => println!("Try again: {} did not match any options", input.as_str()),
		};
	}
}