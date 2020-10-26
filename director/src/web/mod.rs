use std::thread;
use std::time::Duration;

pub struct Web {
	pub online:				bool,
}

impl Web {
	pub fn new() -> Self {

		Web {
			online: false,
		}
	}

	pub fn run(&mut self) {
		self.online = true;

		println!("  Web module started.");

		while self.online {

			thread::sleep(Duration::from_secs(20));
		}
	}
}
