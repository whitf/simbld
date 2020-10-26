use std::thread;
use std::time::Duration;

pub struct Api {
	pub online:				bool,
}

impl Api {
	pub fn new() -> Self {

		Api {
			online: false,
		}
	}

	pub fn run(&mut self) {
		self.online = true;

		println!("  Api module started.");

		while self.online {


			thread::sleep(Duration::from_secs(5));
		}
	}
}
