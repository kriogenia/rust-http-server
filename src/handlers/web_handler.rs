use super::Handler;
use crate::http::{Request, Response, StatusCode};

pub struct WebHandler {

}

impl Handler for WebHandler {
	fn handle_request(&mut self, request: &Request) -> Response {
		println!("{:?}", request);
		Response::new(StatusCode::Ok, Some("<h1>Hello World!</h1>".to_string()))
	}
}