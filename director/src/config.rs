use serde::{Deserialize, Serialize};
use std::fs;
use toml::{map::Map, Value};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Config {
	pub id:							Uuid,
	pub name:						String,
	pub api_address:				String,
	pub api_port:					String,
	pub comm_address:				String,
	pub comm_port:					String,
	pub web_address:				String,
	pub web_port:					String,
	pub db_address:					String,
	pub db_type:					String,
	pub db_user:					String,
	pub db_pass:					String,
	pub db_port:					String,
	pub file:						String,
	pub keyfile:					String,
	pub log_location:				String,
	pub worker_keys:				String,
}

impl Config {
	pub fn new(file: &str) -> Self {

		let toml_content = fs::read_to_string(file).expect("Could not read toml config file.");
		let config_value: Value = toml::from_str(&toml_content).expect("Could not parse config values.");

		let mut id = Uuid::new_v4();
		let mut name = String::from("director_0");
		let mut api_address = String::from("0.0.0.0");
		let mut api_port = String::from("8777");
		let mut comm_address = String::from("0.0.0.0");
		let mut comm_port = String::from("8778");
		let mut web_address = String::from("0.0.0.0");
		let mut web_port = String::from("8080");
		let mut db_address = String::from("0.0.0.0");
		let mut db_type = String::from("internal");
		let mut db_user = String::from("user");
		let mut db_pass = String::from("pass");
		let mut db_port = String::from("5432");
		let mut keyfile = String::from("/etc/simbld/keys/simbld_director");
		let mut log_location = String::from("/var/log/simbld/director");
		let mut worker_keys = String::from("/etc/simbld/keys/workers");

		let config: &toml::map::Map<String, Value> = config_value["simbld_director"].as_table().unwrap();
		for (k, v) in config.iter() {
			let v_str = v.as_str().unwrap().to_string();
			match k.as_str() {
				"id" => {
					id = Uuid::parse_str(&v_str).unwrap();
				},
				"name" => {
					name = v_str;
				},
				"api_address" => {
					api_address = v_str;
				},
				"api_port" => {
					api_port = v_str;
				},
				"comm_address" => {
					comm_address = v_str;
				},
				"comm_port" => {
					comm_port = v_str;
				},
				"db_type" => {
					db_type = v_str;
				},
				"db_user" => {
					db_user = v_str;
				},
				"db_pass" => {
					db_pass = v_str;
				},
				"db_address" => {
					db_address = v_str;
				},
				"db_port" => {
					db_port = v_str;
				},
				"name" => {
					name = v_str;
				},
				"keyfile" => {
					keyfile = v_str;
				},
				"log_location" => {
					log_location = v_str;
				},
				"web_address" => {
					web_address = v_str;
				},
				"web_port" => {
					web_port = v_str;
				},
				"worker_keys" => {
					worker_keys = v_str;
				},
				_ => {
					// Simple ignore any unrecognized options.
					// They will be removed when the data is "written back".
				},
			}
		}

		Config {
			id,
			name,
			api_address,
			api_port,
			comm_address,
			comm_port,
			web_address,
			web_port,
			db_address,
			db_type,
			db_user,
			db_pass,
			db_port,
			file: file.to_string(),
			keyfile,
			log_location,
			worker_keys,
		}
	}

	pub fn save(&mut self) -> bool {

		let mut config_toml = String::from("[simbld_director]");
		config_toml.push('\n');
		config_toml = config_toml + &toml::to_string(&self).unwrap();

		fs::write(self.file.to_string(), config_toml).expect("Could not write to config file.");

		true
	}

}
