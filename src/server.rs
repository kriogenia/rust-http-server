use std::io::{Read};
use std::net::{SocketAddr, TcpListener, TcpStream};
use crate::handlers::Handler;
use crate::http::{HttpError, Request};

const BUFFER_SIZE: usize = 1024;

pub struct Server {
	address: String,
}

impl Server {
	pub fn new(address: String) -> Self {
		Self {
			address
		}
	}

	pub fn run(self, mut handler: impl Handler) {
		let listener = TcpListener::bind(&self.address).unwrap();

		println!("* Listening on {}", self.address);

		loop {
			match listener.accept() {
				Ok(request) => handle_request(request, &mut handler),
				Err(_) => continue
			}
		}
	}
}

fn handle_request((mut stream, address): (TcpStream, SocketAddr), handler: &mut impl Handler) {
	println!("\n> Connection received from {}", address);

	let mut buffer = [0; BUFFER_SIZE];
	let response = match stream.read(&mut buffer) {
		Ok(size) => {
			println!("> Received a request: {}", String::from_utf8_lossy(&buffer[..size]));
			match Request::try_from(&buffer as &[u8]) {
				Ok(request) => {
					handler.handle_request(&request)
				}
				Err(e) => {
					handler.handle_bad_request(&HttpError::from(e))
				}
			}
		}
		Err(_) => {
			handler.handle_bad_request(&HttpError::RequestTimeout)
		}
	};

	println!("< {:?}", response);
	if let Err(e) = response.send(&mut stream) {
		println!("x Failed to send response: {}", e);
	}
}