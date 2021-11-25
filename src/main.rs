mod server;
mod http;
mod handlers;

use server::Server;
use handlers::WebHandler;
use std::env;

fn main() {
	println!("\n* Starting server deployment");

	let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
	let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
	println!("* Server public path: {}", public_path);

	let server = Server::new("127.0.0.1:8080".to_string());
	server.run(WebHandler::new(public_path));
}
