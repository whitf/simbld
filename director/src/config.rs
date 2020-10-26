//use serde::Deserialize;
use std::fs;
//use toml::de::Error;
use uuid::Uuid;

pub struct Config {
	pub id:							Uuid,
	pub name:						String,
	pub api_address:				String,
	pub api_port:					String,
	pub comm_address:				String,
	pub comm_port:					String,
	pub web_address:				String,
	pub web_port:					String,
	pub db_type:					String,
	pub db_user:					String,
	pub db_pass:					String,
	pub db_port:					String,
	pub keyfile:					String,
	pub worker_keys:				String,
}

impl Config {
	pub fn new(file: &str) -> Self {

		let id = Uuid::new_v4();
		let name = String::from("director_0");
		let api_address = String::from("0.0.0.0");
		let api_port = String::from("8777");
		let comm_address = String::from("0.0.0.0");
		let comm_port = String::from("8778");
		let web_address = String::from("0.0.0.0");
		let web_port = String::from("8080");
		let db_type = String::from("internal");
		let db_user = String::from("user");
		let db_pass = String::from("pass");
		let db_port = String::from("5432");
		let keyfile = String::from("/etc/simbld/keys/simbld_director");
		let worker_keys = String::from("/etc/simbld/keys/workers/");

		Config {
			id,
			name,
			api_address,
			api_port,
			comm_address,
			comm_port,
			web_address,
			web_port,
			db_type,
			db_user,
			db_pass,
			db_port,
			keyfile,
			worker_keys,
		}
	}

	pub fn load(&mut self, file: &str) -> bool {
		let _toml_content = fs::read_to_string(file).expect("Error - Failed to read toml config file.");

		true
	}
}