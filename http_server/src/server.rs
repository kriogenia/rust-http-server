use request_handler::handlers::Handler;
use request_handler::http::{HttpError, Request};
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};

const BUFFER_SIZE: usize = 1024;

/// A Server deployable in the given address
pub struct Server {
    address: String,
}

impl Server {
    /// Builds a new server able to run in the given address
    pub fn new(address: String) -> Self {
        Self { address }
    }

    /// Runs the server with the provided request handler
    pub fn run(self, handler: impl Handler) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("* Listening on http://{}", self.address);

        loop {
            match listener.accept() {
                Ok(request) => handle_request(request, &handler),
                Err(_) => continue,
            }
        }
    }
}

fn handle_request((mut stream, address): (TcpStream, SocketAddr), handler: &impl Handler) {
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
