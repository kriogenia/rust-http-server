use request_handler::handler::Handler;
use request_handler::http::{error::HttpError, request::Request};
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

const BUFFER_SIZE: usize = 1024;

/// A Server deployable in the given address
pub struct Server<'a> {
    address: &'a str,
    port: &'a str,
}

impl<'a> Server<'a> {
    /// Builds a new server able to run in the given address
    pub fn new(address: &'a str, port: &'a str) -> Self {
        Self { address, port }
    }

    /// Runs the server with the provided request handler
    pub fn run<H>(self, handler: Arc<H>)
    where
        H: Handler + 'static,
    {
		let address = format!("{}:{}", self.address, self.port);
        let listener = TcpListener::bind(&address).expect("[Unable to bind port]");
        println!("* Listening on http://{}", address);

        loop {
            match listener.accept() {
                Ok(request) => {
                    let handler = handler.clone();
                    thread::spawn(move || {
                        println!("** Thread spawned to attend request");
                        handle_request(request, handler);
                        println!("** Request handled, closing thread")
                    });
                }
                Err(_) => continue,
            }
        }
    }
}

fn handle_request<H: Handler>((mut stream, address): (TcpStream, SocketAddr), handler: Arc<H>) {
    println!("\n> Connection received from {}", address);

    let mut buffer = [0; BUFFER_SIZE];
    let mut response = match stream.read(&mut buffer) {
        Ok(size) => {
            println!(
                "> Received a request: {}",
                String::from_utf8_lossy(&buffer[..size])
            );
            match Request::try_from(&buffer as &[u8]) {
                Ok(request) => {
                    println!("> {:?}", request);
                    handler
                        .handle_request(&request)
                        .unwrap_or_else(|| handler.default_response())
                }
                Err(e) => handler.handle_bad_request(HttpError::from(e)),
            }
        }
        Err(_) => handler.handle_bad_request(HttpError::RequestTimeout),
    };

    if let Err(e) = response.send(&mut stream) {
        println!("x Failed to send response: {}", e);
    }
}
