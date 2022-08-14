mod sotpal;
mod integrations;

use tokio;

use sotpal::Sotpal;
use integrations::run;

fn main() {
	let mut game = Sotpal::new();
	integrations::run(&mut game);
}
