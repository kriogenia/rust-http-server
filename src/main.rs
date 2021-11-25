mod server;
mod http;
mod handlers;

use server::Server;
use handlers::WebHandler;

fn main() {
	let server = Server::new("127.0.0.1:8080".to_string());
	server.run(WebHandler {});
}
