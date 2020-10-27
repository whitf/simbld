use std::sync::mpsc::Receiver;
use std::collections::HashMap;
use uuid::Uuid;

use simbld_models::message::{Message, MessageType};
use simbld_models::worker::Worker;

pub struct Freyr {
	pub online:			bool,
	pub rx:				Receiver<Message>,
	pub workers:		HashMap<Uuid, Worker>,
}

impl Freyr {
	pub fn new(rx: Receiver<Message>) -> Self {

		let workers: HashMap<Uuid, Worker> = HashMap::new();

		Freyr {
			online: false,
			rx,
			workers,
		}
	}

	fn process(&mut self, data: Message) -> Result<i32, &'static str> {

		println!("(freyr) processing a message.");
		println!("workers = {:?}", self.workers);

		match data.message_type {
			MessageType::Online => {
				// register worker
				println!("(freyr) worker online message");

				if let Some(worker_id) = data.worker_id {
					if self.workers.contains_key(&worker_id) {
						println!("updating existing worker");
						let w = self.workers.get_mut(&worker_id).unwrap();
						w.tic();
						w.online();
					} else {
						println!("registering new worker");
						let mut w = Worker::new(worker_id);
						w.tic();
						w.online();
						self.workers.insert(worker_id, w);
					}
				} else {
					println!("worker online message missing worker_id");
				}

				println!("(after online) workers = {:?}", self.workers);

				Ok(0i32)
			},
			MessageType::Offline => {
				// remove worker
				println!("(freyr) worker offline message");
				if let Some(worker_id) = data.worker_id {
					if self.workers.contains_key(&worker_id) {

						let w = self.workers.get_mut(&worker_id).unwrap();
						w.offline();
					}
				}

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
