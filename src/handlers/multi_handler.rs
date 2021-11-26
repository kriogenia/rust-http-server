use crate::handlers::Handler;
use crate::http::{Request, Response};

pub struct MultiHandler {
	handlers: Vec<Box<dyn Handler>>
}

impl MultiHandler {
	pub fn new() -> MultiHandler {
		MultiHandler { handlers: vec![] }
	}

	pub fn add(&mut self, handler: Box<dyn Handler>) {
		self.handlers.push(handler);
	}
}

impl Handler for MultiHandler {
	fn handle_request(&mut self, request: &Request) -> Option<Response> {
		for handler in self.handlers.iter_mut() {
			if let Some(res) = handler.handle_request(request) {
				return Some(res);
			}
		}
		None
	}

	fn default_response(&self) -> Response {
		self.handlers[0].default_response()
	}

}