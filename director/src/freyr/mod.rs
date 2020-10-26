use std::thread;
use std::time::Duration;

pub struct Freyr {
	pub online:		bool,
}

impl Freyr {
	pub fn new() -> Self {

		Freyr {
			online: false,
		}
	}

	pub fn run(&mut self) {
		self.online = true;

		println!("  Freyr module started.");

		while self.online {
			thread::sleep(Duration::from_secs(12));
		}
	}
}
