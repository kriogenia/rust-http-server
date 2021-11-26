use crate::FileReader;
use crate::handlers::Handler;
use crate::http::{Request, Response, StatusCode};

const API_PATH: &str = "api/";

pub struct ApiHandler {
	fs: Box<FileReader>,
}

impl ApiHandler {
	pub fn new(fs: Box<FileReader>) -> Self {
		Self { fs }
	}
}

impl Handler for ApiHandler {
	fn handle_request(&self, request: &Request) -> Option<Response> {
		if request.path().starts_with(API_PATH) {
			None
		} else { None }
	}

	fn default_response(&self) -> Response {
		Response::new(StatusCode::NotFound,
		              to_json_message(StatusCode::NotFound.to_string()))
	}
}

fn to_json_message(message: &str) -> Option<String> {
	Some(format!("{{\"message\":\"{}\"}}", message))
}