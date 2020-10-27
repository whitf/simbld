use bincode;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::str::from_utf8;
use std::time::Duration;

use simbld_models::message::{Message, MessageType};

pub fn handle_worker_request(mut stream: TcpStream) {
	let mut data = [0 as u8; 64];

	match stream.read(&mut data) {
		Ok(size) => {
			println!("received data");
			println!("size = {}", size);

			let msg: Message = bincode::deserialize(&data).unwrap();

			println!("message = {:?}", msg);

			match msg.message_type {
				MessageType::Online => {
					println!("handling worker online message");
				},
				MessageType::Offline => {
					println!("handling worker offline message");
				},
				MessageType::Job => {
					println!("handling job-related message");
				},
				_ => {
					println!("some other message");
				},
			}






			let mut response = Message::new(MessageType::Response);
			response.body = Some(String::from("Okay"));

			let bytes = bincode::serialize(&response).unwrap();

			stream.write(&bytes).unwrap();
		},
		Err(e) => {
			println!("error = {}", e);
			stream.shutdown(Shutdown::Both).unwrap();
		}
	}
}

pub struct Communication {
	pub online:						bool,
	pub address:					String,
	pub port:						String,
}

impl Communication {
	pub fn new(address: String, port: String) -> Self {
		Communication {
			online: false,
			address,
			port,
		}
	}

	pub fn run(&mut self) {
		self.online = true;

		let tcp_listener = format!("{}:{}", self.address, self.port);
		let listener = TcpListener::bind(tcp_listener).unwrap();

		for stream in listener.incoming() {
			match stream {
				Ok(stream) => {
					thread::spawn(move || {
						handle_worker_request(stream)
					});
				}
				Err(e) => {
					println!("Tcp connection error: {}", e);
				}
			}
		}

		drop(listener);
	}
}
