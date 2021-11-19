use std::io::Read;
use std::net::TcpListener;

const BUFFER_SIZE: usize = 1024;

pub struct Server {
	address: String
}

impl Server {

	pub fn new(address: String) -> Self {
		Self {
			address
		}
	}

	pub fn run(self) {
		let listener = TcpListener::bind(&self.address).unwrap() ;

		println!("Listening on {}", self.address);

		loop {
			match listener.accept() {
				Ok((mut stream, address)) => {
					println!("\nConnection received from {}", address);

					let mut buffer = [0; BUFFER_SIZE];
					match stream.read(&mut buffer) {
						Ok(_) => {
							println!("Received a request: {}", String::from_utf8_lossy(&buffer));
						},
						Err(e) => println!("Fail reading from connection: {}", e)
					}
				},
				Err(e) => continue
			}
		}
	}

}
