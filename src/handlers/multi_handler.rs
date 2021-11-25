use crate::handlers::Handler;
use crate::http::{Request, Response};

pub struct MultiHandler {
	handlers: Vec<Box<dyn Handler>>
}

impl MultiHandler {
	pub fn new() {
		MultiHandler { handlers: vec![] };
	}

	pub fn add(&mut self, handler: Box<dyn Handler>) {
		self.handlers.push(handler);
	}
}

impl Handler for MultiHandler {
	fn handle_request(&mut self, request: &Request) -> Option<Response> {
		None
	}
}