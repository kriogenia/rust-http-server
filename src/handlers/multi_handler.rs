use crate::handlers::Handler;
use crate::http::{Request, Response};

/// Composite handler to allow the use of multiple Handlers.
/// <i>A possible feature to thsi is adding the option to use an optional root path for the handler
/// allowing to nest handlers to manage path ramifications</i>
pub struct MultiHandler {
    handlers: Vec<Box<dyn Handler>>,
}

impl MultiHandler {
    pub fn new() -> MultiHandler {
        MultiHandler { handlers: vec![] }
    }

    /// Adds a new Handler.
    /// <i>Right now, no way of remove exists as my current implementation wouldn't need it
    /// and it would be going overboard</i>
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
