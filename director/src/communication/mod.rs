use std::thread;
use std::time::Duration;

pub struct Communication {
	pub online:						bool,
}

impl Communication {
	pub fn new() -> Self {
		Communication {
			online: false,
		}
	}

	pub fn run(&mut self) {
		self.online = true;

		println!("  Communication module started.");

		while self.online {
			thread::sleep(Duration::from_secs(10));
		}
	}
}
