use crate::http::{HttpError, Request, Response};

pub trait Handler {
	fn handle_request(&mut self, request: &Request) -> Response;

	fn handle_bad_request(&mut self, e: &HttpError) -> Response {
		println!("Error managing response: {}", e);
		Response::new(e.code(), Some(e.message().to_string()))
	}
}