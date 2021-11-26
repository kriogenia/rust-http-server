use crate::http::{HttpError, Request, Response};

pub trait Handler {
	fn handle_request(&self, request: &Request) -> Option<Response>;

	fn handle_bad_request(&self, e: HttpError) -> Response {
		eprintln!("Error managing response: {}", e);
		let message = e.message().to_string();
		Response::new(e.into(), Some(message))
	}
}