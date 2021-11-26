use crate::fs::FileReader;
use super::Handler;
use crate::http::{Method, Request, Response, StatusCode};

pub struct WebHandler {
	fs: Box<FileReader>
}

impl WebHandler {
	pub fn new(fs: Box<FileReader>) -> Self {
		WebHandler { fs }
	}

	fn get_router(&self, path: &str) -> Option<Response> {
		match path {
			"/" => ok_response(self.fs.read_file("index.html")),
			"/hello" => ok_response(self.fs.read_file("helloworld.txt")),
			_ => self.fs.read_file(path).and_then(|s| ok_response(Some(s)))
		}
	}

}

impl Handler for WebHandler {
	fn handle_request(&self, request: &Request) -> Option<Response> {
		println!("> {:?}", request);

		match request.method() {
			Method::GET => self.get_router(request.path()),
			_ => None
		}
	}

	fn default_response(&self) -> Response {
		Response::new(StatusCode::NotFound, self.fs.read_file("404.html"))
	}
}

fn ok_response(content: Option<String>) -> Option<Response> {
	Some(Response::new(StatusCode::Ok, content))
}