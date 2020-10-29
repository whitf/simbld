use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::module::ModuleName;

#[derive(Clone, Debug, Deserialize, Serialize, ToString)]
pub enum LogType {
	Access,
	Critical,
	Error,
	System,
	StdOut,
	StdErr,
	Warning,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Log {
	pub message:				String,
	pub module:					ModuleName,
	pub date_time:				SystemTime,
	pub log_type:				LogType,
}

impl Log {
	pub fn new(module: ModuleName, log_type: LogType, message: String) -> Self {
		let date_time = SystemTime::now();

		Log {
			message,
			module,
			log_type,
			date_time,
		}
	}
}
