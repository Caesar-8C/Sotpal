mod terminal;
mod telegram;

use crate::Sotpal;

pub fn run() {
	if cfg!(feature = "terminal") {
		terminal::run();
	}
	else if cfg!(feature = "telegram") {
		telegram::run();
	}
	else {
		println!("No integration has been enabled");
	}
}