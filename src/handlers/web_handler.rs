use super::Handler;
use crate::http::{Method, Request, Response, StatusCode};

pub struct WebHandler {}

impl Handler for WebHandler {
	fn handle_request(&mut self, request: &Request) -> Response {
		println!("{:?}", request);

		match request.method() {
			Method::GET => get_router(request.path()),
			_ => not_found_response()
		}
	}
}

fn get_router(path: &str) -> Response {
	match path {
		"/" => ok_response("<h1>Hi</h1><p>And welcome to my Rust HTTP Server</p>".to_string()),
		"/hello" => ok_response("<h1>Hello World!</h1>".to_string()),
		_ => not_found_response()
	}
}

fn ok_response(body: String) -> Response {
	Response::new(StatusCode::Ok, Some(body))
}

fn not_found_response() -> Response {
	Response::new(StatusCode::NotFound, Some("<h1>404</h1>".to_string()))
}