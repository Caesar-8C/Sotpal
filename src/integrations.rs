mod telegram;
use telegram::run;

#[cfg(terminal)]
mod terminal;
#[cfg(terminal)]
use terminal::run;

pub fn run_integration() {
	run();
}