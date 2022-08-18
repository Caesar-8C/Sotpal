mod sotpal;
mod utils;
mod integrations;

use sotpal::Sotpal;
use integrations::telegram::run;

fn main() {
	run();
}
