use tokio;
mod sotpal;
use sotpal::Sotpal;

fn main() {
	println!("Hello, world!");
	let mut game = Sotpal::new();
	let gizka_id = game.add_player("Gizka".to_string());
	game.add_player("".to_string());
	let pukhlik_id = game.add_player("Pukhlik".to_string());
	game.add_topic(gizka_id, "my first topic".to_string());
	game.add_topic(pukhlik_id, "".to_string());
	game.print_players();
	game.list_topics();
}
