use bincode;
use serde::Serializer;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;
use uuid::Uuid;

use simbld_models::message;

fn main() {

	match TcpStream::connect("localhost:8778") {
		Ok(mut stream) => {
			println!("connected to localhost:8778");

			//let msg = b"HelloFromTheOtherSide";

			let mut msg = message::Message::new(message::MessageType::Online);
			let worker_id = Uuid::parse_str("46a8f04b-bedb-4941-a85a-121a7dee1179").unwrap();
			msg.worker_id = Some(worker_id);

			//let bytes:Vec<u8> = bincode::serialize(&msg).unwrap(); 

			println!("message = {:?}", msg);

			let bytes: Vec<u8> = bincode::serialize(&msg).unwrap();

			stream.write(&bytes).unwrap();
			println!("Sent hello message, awaiting response...");

			let mut data = [0 as u8; 64];

			match stream.read(&mut data) {
				Ok(size) => {
					println!("size is {:?}", size);
					let msg: message::Message = bincode::deserialize(&data).unwrap();

					println!("msg = {:?}", msg);



					match msg.body {
						Some(val) => {
							println!("val = {}", val);
							println!("Okay response received from director.");
						},
						None => {
							println!("Blank response body received from director.");
						},
						_ => {
							println!("Unrecognized response from director.");
						}
					}
				},
				Err(e) => {
					println!("Failed to receive data: {}", e);

					println!(" data (maybe) = {:?}", data);
				}
			}
		},
		Err(e) => {
			println!("Failed to connect: {}", e);
		}
	}

    println!("terminated.");
}
