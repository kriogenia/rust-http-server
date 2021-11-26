mod server;
mod http;
mod handlers;
mod fs;

use server::Server;
use handlers::WebHandler;
use std::env;
use crate::fs::FileReader;
use crate::handlers::{MultiHandler};

const IP: &str = "127.0.0.1";
const PORT: &str = "8080";

/// Launching point of the application
fn main() {
	println!("\n* Starting server deployment");

	let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
	let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
	println!("* Server public path: {}", public_path);

	let file_reader = Box::new(FileReader::new(public_path));
	// Set handlers
	let mut root_handler = MultiHandler::new();
	root_handler.add(Box::new(WebHandler::new(file_reader)));
	// Launch server
	let server = Server::new(format!("{}:{}", IP, PORT));
	server.run(root_handler);
}
