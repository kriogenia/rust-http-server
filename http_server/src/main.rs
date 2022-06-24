use prelude::*;

use std::{env, sync::Arc};

use file_system::fs::FileReader;
use request_handler::handlers::{root::RootHandler, web::WebHandler, Handler};
use server::Server;

mod prelude;
mod server;

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
    let handlers: Vec<Box<dyn Handler>> = vec![Box::new(web_handler)];
    let root_handler = RootHandler::new(handlers);

    // Launch server
    let server = Server::new(&address, &port);
    server.run(Arc::new(root_handler));
}
