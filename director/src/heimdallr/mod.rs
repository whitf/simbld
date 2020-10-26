use std::thread;
use std::time::Duration;

pub struct Heimdallr {
	online:			bool,
}

impl Heimdallr {
	pub fn new() -> Self {

		Heimdallr {
			online: false,
		}
	}

	pub fn run(&mut self) {
		self.online = true;

		println!("  Heimdallr module started.");

		while self.online {
			thread::sleep(Duration::from_secs(7));
		}
	}
}
