use crate::FileReader;
use crate::handlers::Handler;
use crate::http::{Method, QueryMap, Request, Response, StatusCode};

const API_PATH: &str = "/api";
const NAME_PARAM: &str = "name";

pub struct ApiHandler {
	fs: Box<FileReader>,
}

impl ApiHandler {
	pub fn new(fs: Box<FileReader>) -> Self {
		Self { fs }
	}

	fn get_router(&self, path: &str, request: &Request) -> Option<Response> {
		match path {
			"/hello" => self.get_hello(request.query()),
			_ => None
		}
	}

	fn get_hello(&self, queries: Option<&QueryMap>) -> Option<Response> {
		if queries.is_some() {
			if let Some(name) = queries.unwrap().get(NAME_PARAM) {
				return ok_response( to_json_message(&format!("Hello {}!!!", name)));
			}
		}
		ok_response(self.fs.read_file("helloworld.txt")
				.and_then(|s| to_json_message(&s)))
	}
}

impl Handler for ApiHandler {
	fn handle_request(&self, request: &Request) -> Option<Response> {
		dbg!("Request: {}", request);
		// Works only in the API path
		if request.path().starts_with(API_PATH) {
			let api_path = &request.path()[API_PATH.bytes().count()..];
			match request.method() {
				Method::GET => self.get_router(api_path, request),
				//Method::POST => self.post_router(request.path()),
				//Method::DELETE => self.delete_router(request.path()),
				_ => None
			}
		} else { None }
	}

	fn default_response(&self) -> Response {
		Response::new(StatusCode::NotFound,
		              to_json_message(StatusCode::NotFound.to_string()))
	}
}

fn ok_response(content: Option<String>) -> Option<Response> {
	Some(Response::new(StatusCode::Ok, content))
}

fn to_json_message(message: &str) -> Option<String> {
	Some(format!("{{\"message\":\"{}\"}}", message))
}