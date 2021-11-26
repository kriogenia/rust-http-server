use std::io::{Read};
use std::net::{SocketAddr, TcpListener, TcpStream};
use crate::handlers::Handler;
use crate::http::{HttpError, Request, Response, StatusCode};

const BUFFER_SIZE: usize = 1024;

/// A Server deployable in the given address
pub struct Server {
	address: String,
}

impl Server {
	/// Builds a new server able to run in the given address
	pub fn new(address: String) -> Self {
		Self {
			address
		}
	}

	/// Runs the server with the provided request handler
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
					handler.handle_request(&request).unwrap_or(
						Response::new(StatusCode::NotFound, None))
				}
				Err(e) => {
					handler.handle_bad_request(HttpError::from(e))
				}
			}
		}
		Err(_) => {
			handler.handle_bad_request(HttpError::RequestTimeout)
		}
	};

	println!("< {:?}", response);
	if let Err(e) = response.send(&mut stream) {
		println!("x Failed to send response: {}", e);
	}
}