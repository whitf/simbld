use bincode;
use serde::Serializer;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use uuid::Uuid;

use simbld_models::message::{Message, MessageType};
use simbld_models::log::{Log, LogType};

pub fn heartbeat(conn_str: String, stx: Sender<Message>) {
	loop {
		println!("- tic");

		match TcpStream::connect("localhost:8778") {
			Ok(mut stream) => {

				let mut msg = Message::new(MessageType::Online);
				let worker_id = Uuid::parse_str("46a8f04b-bedb-4941-a85a-121a7dee1179").unwrap();
				msg.worker_id = Some(worker_id);

				println!("message = {:?}", msg);

				let bytes: Vec<u8> = bincode::serialize(&msg).unwrap();
				stream.write(&bytes).unwrap();

				let mut data = [0 as u8, 128];

				match stream.read(&mut data) {
					Ok(size) => {
						println!("received reply from director (size = {}).", size);
					},
					Err(e) => {
						println!("failed to received data from director: {}", e);
					}
				}
				
				stream.shutdown(Shutdown::Both).expect("shutdown call failed");
			},
			Err(e) => {
				println!("failed to connect to director: {}", e);
			}
		}


		thread::sleep(Duration::from_secs(10));
	}
}

pub struct Bifrost {
	brx:					Receiver<Message>,
	ltx:					Sender<Log>,
	stx:					Sender<Message>,
	online:					bool,
	worker_id:				Uuid,
}

impl Bifrost {

	//pub fn new(worker_id: Uuid, ltx: Sender<Log>, brx: Receiver<Message>) -> Self {
	pub fn new(ltx: Sender<Log>, stx: Sender<Message>, brx: Receiver<Message>) -> Self {
		let online = false;
		let worker_id = Uuid::new_v4();

		Bifrost {
			brx,
			ltx,
			stx,
			online,
			worker_id,
		}
	}

	pub fn run(&mut self) {
		self.online = true;

		let conn_str = String::from("localhost::8778");

		let stx = self.stx.clone();
		let heartbeat_handler = thread::spawn(move || {
			heartbeat(conn_str, stx);
		});

		while self.online {
			let data = self.brx.recv();


			println!("worker - bifrost")

		}

		heartbeat_handler.join().unwrap();
	}
}