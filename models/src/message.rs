use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

use crate::job;

#[derive(Debug, Deserialize, Serialize)]
pub enum MessageType {
	Job,
	Online,
	Offline,
	Response,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
	pub body:						Option<String>,
	pub id:							Uuid,
	pub job_id:						Option<Uuid>,
	pub job_status:					Option<job::JobStatus>,
	pub message_type:				MessageType,
	pub timestamp:					SystemTime,
	pub worker_id:					Option<Uuid>,
}

impl Message {

	pub fn new(message_type: MessageType) -> Self {

		let body = None;
		let id = Uuid::new_v4();
		let job_id = None;
		let job_status = None;
		let timestamp = SystemTime::now();
		let worker_id = None;

		Message {
			body,
			id,
			job_id,
			job_status,
			message_type,
			timestamp,
			worker_id,
		}
	}

}
