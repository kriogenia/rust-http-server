use crate::fs::FileReader;
use super::Handler;
use crate::http::{Header, Method, Request, Response, StatusCode};

/// Handler in charge of serving all the web resources and endpoints.
/// This Handler
pub struct WebHandler {
	fs: Box<FileReader>
}

impl WebHandler {
	pub fn new(fs: Box<FileReader>) -> Self {
		Self { fs }
	}

	/// GET web requests
	fn get_router(&self, path: &str) -> Option<Response> {
		match path {
			"/" => ok_response(self.fs.read_file("index.html")),
			"/hello" => ok_response(self.fs.read_file("helloworld.txt")
				.and_then(|s| Some(format!("<p>{}</p>", s))
			)),
			_ => self.fs.read_file(path).and_then(
				|s| Some(Response::new(StatusCode::Ok, Some(s))))
		}
	}

}

impl Handler for WebHandler {
	fn handle_request(&mut self, request: &Request) -> Option<Response> {
		match request.method() {
			Method::GET => self.get_router(request.path()),
			_ => None
		}
	}

	fn default_response(&self) -> Response {
		Response::new(StatusCode::NotFound, self.fs.read_file("404.html"))
	}
}

/// Build an HTML 200 Response with the given body
fn ok_response(content: Option<String>) -> Option<Response> {
	let mut response = Response::new(StatusCode::Ok, content);
	response.header(Header::ContentType, "text/html");
	Some(response)
}