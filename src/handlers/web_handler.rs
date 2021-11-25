use crate::fs::FileReader;
use super::Handler;
use crate::http::{Method, Request, Response, StatusCode};

pub struct WebHandler<'fr> {
	fs: &'fr FileReader<'fr>
}

impl<'fr> WebHandler<'fr> {
	pub fn new(fs: &'fr FileReader) -> Self {
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

impl<'fr> Handler for WebHandler<'fr> {
	fn handle_request(&mut self, request: &Request) -> Option<Response> {
		println!("> {:?}", request);

		match request.method() {
			Method::GET => self.get_router(request.path()),
			_ => None
			//_ => not_found_response(self.read_file("404.html"))
		}
	}
}

fn ok_response(content: Option<String>) -> Option<Response> {
	Some(Response::new(StatusCode::Ok, content))
}

fn not_found_response(content: Option<String>) -> Response {
	Response::new(StatusCode::NotFound, content)
}