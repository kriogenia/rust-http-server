use crate::handler::Handler;
use crate::http::{Request, Response};

/// Composite handler to allow the use of multiple Handlers.
/// <i>A possible feature to thsi is adding the option to use an optional root path for the handler
/// allowing to nest handlers to manage path ramifications</i>
pub struct RootHandler {
    handlers: Vec<Box<dyn Handler>>,
}

impl RootHandler {
    pub fn new(handlers: Vec<Box<dyn Handler>>) -> RootHandler {
        RootHandler { handlers }
    }

}

impl Handler for RootHandler {
    fn handle_request(&self, request: &Request) -> Option<Response> {
        for handler in self.handlers.iter() {
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
