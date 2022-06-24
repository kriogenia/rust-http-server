mod server;

use std::env;

use file_system::fs::FileReader;
use request_handler::handlers::{root::RootHandler, web::{WebHandler, self}};
use server::Server;

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

    // Set handlers
    let web_handler = WebHandler::new(Box::new(FileReader::new(&public_path)));
    //let handlers = vec![Box::new(web_handler)];
    //let mut root_handler = RootHandler::new(handlers);

    // Launch server
    let address = format!("{}:{}", address, port);
    let server = Server::new(&address);
    server.run(Box::new(web_handler));
}
