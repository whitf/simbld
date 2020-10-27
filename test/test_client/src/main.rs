use bincode;
use serde::Serializer;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;

use simbld_models::message;

fn main() {

	match TcpStream::connect("localhost:8778") {
		Ok(mut stream) => {
			println!("connected to localhost:8778");

			//let msg = b"HelloFromTheOtherSide";

			let msg = message::Message::new(message::MessageType::Online);

			//let bytes:Vec<u8> = bincode::serialize(&msg).unwrap(); 

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
