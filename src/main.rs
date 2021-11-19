fn main() {
	let server = Server::new("127.0.0.1:8080".to_string());
	server.run();
}

struct Server {

	address: String

}

impl Server {

	fn new(address: String) -> Self {
		Self {
			address
		}
	}

	fn run(self) {
		println!("Listening on {}",self.address);
	}

}

struct Request {

	path: String,
	query_string: String,
	method: Method

}

enum Method {
	GET,
	DELETE,
	POST,
	PUT,
	HEAD,
	CONNECT,
	OPTIONS,
	TRACE,
	PATCH
}