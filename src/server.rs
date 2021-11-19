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
		println!("Listening on {}",self.address);
	}

}
