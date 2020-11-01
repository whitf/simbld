use bincode;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::sync::mpsc::{Receiver, Sender};

use simbld_models::job::Job;
use simbld_models::message::{Message, MessageType, ResponseType};
use simbld_models::module::ModuleName;
use simbld_models::log::{Log, LogType};

pub fn handle_worker_request(mut stream: TcpStream, ftx: Sender<Message>) {
	let mut data = [0 as u8; 128];

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
	pub address:					String,
	pub ftx:						Sender<Message>,
	pub jrx:						Receiver<Job>,	
	pub ltx:						Sender<Log>,
	pub online:						bool,
	pub port:						String,
}

impl Communication {
	pub fn new(address: String, port: String, ltx: Sender<Log>, ftx: Sender<Message>, jrx: Receiver<Job>) -> Self {
		let ftx = ftx.clone();

		Communication {
			address,
			ftx,
			jrx,
			ltx,
			online: false,
			port,
		}
	}

	pub fn run(&mut self) {
		self.online = true;

		self.ltx.send(Log::new(ModuleName::Communication, LogType::System, String::from("Communication module online."))).unwrap();

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
