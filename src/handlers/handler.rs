use crate::http::{HttpError, Request, Response, StatusCode};

/// Receives and handles HTTP requests. It can return a Response if it contains the logic
/// to address the request.
pub trait Handler {
	/// Handles a new valid request and returns the Response to send back if it knows how to manage it
	fn handle_request(&mut self, request: &Request) -> Option<Response>;

	/// Generates a default response that can be used as placeholder or as a backing response when the
	/// proper handle_request fails to deliver a response
	fn default_response(&self) -> Response {
		Response::new(StatusCode::NotFound, None)
	}

	/// Handles an invalid request and returns the Response to send back in those cases
	fn handle_bad_request(&self, e: HttpError) -> Response {
		eprintln!("Error managing response: {}", e);
		let message = e.message().to_string();
		Response::new(e.into(), Some(message))
	}
}