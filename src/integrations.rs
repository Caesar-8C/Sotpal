mod terminal;

use crate::Sotpal;

pub fn run(game: &mut Sotpal) {
	if cfg!(feature = "terminal") {
		terminal::run(game);
	}
	else {
		println!("No integration has been enabled");
	}
}