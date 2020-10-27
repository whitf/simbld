use std::sync::mpsc::Receiver;

use simbld_models::message::{Message, MessageType};

pub struct Freyr {
	pub online:			bool,
	pub rx:				Receiver<Message>,
}

impl Freyr {
	pub fn new(rx: Receiver<Message>) -> Self {

		Freyr {
			online: false,
			rx,
		}
	}

	fn process(&mut self, data: Message) -> Result<i32, &'static str> {

		println!("(freyr) processing a message.");

		match data.message_type {
			MessageType::Online => {
				// register worker
				println!("(freyr) worker online message");

				Ok(0i32)
			},
			MessageType::Offline => {
				// remove worker
				println!("(freyr) worker offline message");

				Ok(0i32)
			},
			MessageType::Job => {
				// Process job related events.
				println!("(freyr) working job message");

				Ok(0i32)
			},
			_ => Err("Unrecognized message_type."),
		}
	}

	pub fn run(&mut self) {
		self.online = true;

		println!("  Freyr module started.");

		while self.online {

			let data = self.rx.recv();

			if let Err(e) = self.process(data.unwrap()) {
				// I should be a log message....
				println!("(freyr) Message processing error : {:?}", e)
			}
		}
	}
}
