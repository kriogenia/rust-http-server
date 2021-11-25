use crate::http::{HttpError, Request, Response};

pub trait Handler {
	fn handle_request(&mut self, request: &Request) -> Response;

	fn handle_bad_request(&mut self, e: HttpError) -> Response {
		eprintln!("Error managing response: {}", e);
		let message = e.message().to_string();
		Response::new(e.into(), Some(message))
	}
}