mod server;
mod http;
mod handlers;
mod fs;

use server::Server;
use handlers::WebHandler;
use std::env;
use crate::fs::FileReader;
use crate::handlers::{MultiHandler};

fn main() {
	println!("\n* Starting server deployment");

	let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
	let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
	println!("* Server public path: {}", public_path);

	let file_reader = FileReader::new(&public_path);

	let _root_handler = MultiHandler::new();

	let server = Server::new("127.0.0.1:8080".to_string());
	server.run(WebHandler::new(&file_reader));
}
