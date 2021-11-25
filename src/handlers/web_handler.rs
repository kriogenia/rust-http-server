use super::Handler;
use crate::http::{Method, Request, Response, StatusCode};
use std::fs;

pub struct WebHandler {
	public_path: String
}

impl WebHandler {
	pub fn new(public_path: String) -> Self {
		WebHandler { public_path }
	}

	fn read_file(&self, file_path: &str) -> Option<String> {
		let path = format!("{}/{}", self.public_path, file_path);
		match fs::canonicalize(path) {
			Ok(path) => {
				if path.starts_with(&self.public_path) {
					fs::read_to_string(path).ok()
				} else {
					eprintln!("Directory Traversal Attack attempted: {}", file_path);
					None
				}
			},
			Err(e) => None
		}
	}

	fn get_router(&self, path: &str) -> Response {
		match path {
			"/" => ok_response(self.read_file("index.html")),
			"/hello" => ok_response(self.read_file("html/helloworld.txt")),
			_ => match self.read_file(path) {
				Some(s) => ok_response(Some(s)),
				None => not_found_response(self.read_file("404.html"))
			}
		}
	}


}

impl Handler for WebHandler {
	fn handle_request(&mut self, request: &Request) -> Response {
		println!("> {:?}", request);

		match request.method() {
			Method::GET => self.get_router(request.path()),
			_ => not_found_response(self.read_file("404.html"))
		}
	}
}

fn ok_response(content: Option<String>) -> Response {
	Response::new(StatusCode::Ok, content)
}

fn not_found_response(content: Option<String>) -> Response {
	Response::new(StatusCode::NotFound, content)
}