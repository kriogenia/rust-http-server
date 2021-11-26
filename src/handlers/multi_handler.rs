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
	fn handle_request(&self, request: &Request) -> Option<Response> {
		for handler in self.handlers.iter() {
			if let Some(res) = handler.handle_request(request) {
				return Some(res);
			}
		}
		None
	}
}