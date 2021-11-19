use std::net::TcpListener;

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
				Ok((_stream, address)) => {
					println!("Connection received from {}", address);
				},
				Err(e) => continue
			}
		}
	}

}
