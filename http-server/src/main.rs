mod fs;
mod handlers;
mod http;
mod server;

use std::env;

const ADDRESS_VAR: &str = "RUST_SERVER_ADDRESS";
const PORT_VAR: &str = "RUST_SERVER_PORT";
const PUBLIC_PATH_VAR: &str = "RUST_SERVER_PUBLIC_PATH";

const DEFAULT_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8080";

/// Launching point of the application
fn main() {
    println!("\n* Starting server deployment");

    let address = env::var(ADDRESS_VAR).unwrap_or_else(|_| DEFAULT_ADDRESS.to_string());
    let port = env::var(PORT_VAR).unwrap_or_else(|_| DEFAULT_PORT.to_string());

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var(PUBLIC_PATH_VAR).unwrap_or(default_path);
    println!("* Server public path: {}", public_path);

	/*
    // Set handlers
    let mut root_handler = MultiHandler::new();
    root_handler.add(Box::new(WebHandler::new(Box::new(FileReader::new(
        &public_path,
    )))));
    root_handler.add(Box::new(ApiHandler::new(Box::new(FileReader::new(
        &public_path,
    )))));

    // Launch server
    let server = Server::new(format!("{}:{}", address, port));
    server.run(root_handler);
	 */
}
