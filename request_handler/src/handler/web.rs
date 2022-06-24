use std::sync::Arc;

use super::Handler;
use file_system::FileReader;
use crate::http::{Header, Method, Request, Response, StatusCode};

/// Handler in charge of serving all the web resources and endpoints.
/// This Handler
pub struct WebHandler {
    fs: Arc<FileReader>,
}

impl WebHandler {
    pub fn new(fs: Arc<FileReader>) -> Self {
        Self { fs }
    }

    /// GET web requests
    fn get_router(&self, path: &str) -> Option<Response> {
        match path {
            "/" => ok_response(self.fs.read_file("index.html")),
            "/hello" => ok_response(
                self.fs
                    .read_file("helloworld.txt")
                    .map(|s| format!("<p>{}</p>", s)),
            ),
            _ => self.fs
                .read_file(path)
                .map(|s| Response::new(StatusCode::Ok, Some(s))),
        }
    }
}

impl Handler for WebHandler {
    fn handle_request(&self, request: &Request) -> Option<Response> {
        match request.method() {
            Method::Get => self.get_router(request.path()),
            _ => None,
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
