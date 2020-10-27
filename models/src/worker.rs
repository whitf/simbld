use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Worker {
	pub id:						Uuid,
	pub online:					bool,
	pub last_heartbeat:			Option<SystemTime>,
}

impl Worker {
	pub fn new(id: Uuid) -> Self {

		let online = false;
		let last_heartbeat = None;

		Worker {
			id,
			online,
			last_heartbeat,
		}
	}

	pub fn tic(&mut self) {
		self.last_heartbeat = Some(SystemTime::now());
	}

	pub fn online(&mut self) {
		self.online = true;
	}

	pub fn offline(&mut self) {
		self.online = false;
	}
}
