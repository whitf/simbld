use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::str::from_utf8;
use std::time::Duration;

pub fn handle_worker_request(mut stream: TcpStream) {
	let mut data = [0 as u8; 50];

	match stream.read(&mut data) {
		Ok(size) => {
			println!("received data");
			println!("size = {}", size);

			stream.write(&data[0..size]).unwrap();
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
