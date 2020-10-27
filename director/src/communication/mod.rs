use bincode;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::sync::mpsc::Sender;

use simbld_models::message::{Message, MessageType, ResponseType};

pub fn handle_worker_request(mut stream: TcpStream, ftx: Sender<Message>) {
	let mut data = [0 as u8; 64];

	match stream.read(&mut data) {
		Ok(_) => {
			let msg: Message = bincode::deserialize(&data).unwrap();

			//println!("message = {:?}", msg);

			ftx.send(msg).unwrap();

			let mut response = Message::new(MessageType::Response);
			response.response_type = Some(ResponseType::Received);

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
	pub ftx:						Sender<Message>,
}

impl Communication {
	pub fn new(address: String, port: String, ftx: Sender<Message>) -> Self {
		let ftx = ftx.clone();

		Communication {
			online: false,
			address,
			port,
			ftx,
		}
	}

	pub fn run(&mut self) {
		self.online = true;

		while self.online {

			let tcp_listener = format!("{}:{}", self.address, self.port);
			let listener = TcpListener::bind(tcp_listener).unwrap();

			for stream in listener.incoming() {
				match stream {
					Ok(stream) => {
						let ftx = self.ftx.clone();
						thread::spawn(move || {
							handle_worker_request(stream, ftx)
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
}
