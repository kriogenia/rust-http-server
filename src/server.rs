use std::io::{Read};
use std::net::{SocketAddr, TcpListener, TcpStream};
use crate::http::{Request, Response, StatusCode};

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

	pub fn run(self) {
		let listener = TcpListener::bind(&self.address).unwrap();

		println!("Listening on {}", self.address);

		loop {
			match listener.accept() {
				Ok(request) => handle_request(request),
				Err(_) => continue
			}
		}
	}
}

fn handle_request((mut stream, address): (TcpStream, SocketAddr)) {
	println!("\nConnection received from {}", address);

	let mut buffer = [0; BUFFER_SIZE];
	let response = match stream.read(&mut buffer) {
		Ok(size) => {
			println!("Received a request: {}", String::from_utf8_lossy(&buffer[..size]));
			match Request::try_from(&buffer as &[u8]) {
				Ok(request) => {
					println!("{:?}", request);
					Response::new(StatusCode::Ok, Some("<h1>Hello World!</h1>".to_string()))
				}
				Err(e) => {
					println!("Error converting byte array: {}", e);
					Response::new(StatusCode::BadRequest, None)
				}
			}
		}
		Err(e) => {
			println!("Fail reading from connection: {}", e);
			Response::new(StatusCode::RequestTimeout, None)
		}
	};

	if let Err(e) = response.send(&mut stream) {
		println!("Failed to send response: {}", e);
	}
}