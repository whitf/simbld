use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {

	match TcpStream::connect("localhost:8778") {
		Ok(mut stream) => {
			println!("connected to localhost:8778");

			let msg = b"HelloFromTheOtherSide";

			stream.write(msg).unwrap();
			println!("Sent hello message, awaiting response...");

			let mut data = [0 as u8; 21];

			match stream.read_exact(&mut data) {
				Ok(_) => {
					//let text = from_utf8(&data).unwrap();
					//println!("reply: {}", text);

					if &data == msg {
						println!("reply is ok!");
					} else {
						let text = from_utf8(&data).unwrap();
						println!("Unexpected reply: {}", text);
					}
				},
				Err(e) => {
					println!("Failed to receive data: {}", e);
				}
			}
		},
		Err(e) => {
			println!("Failed to connect: {}", e);
		}
	}

    println!("terminated.");
}
