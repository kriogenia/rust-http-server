use crate::handlers::Handler;
use crate::http::{Header, Method, QueryMap, QueryValue, Request, Response, StatusCode};
use crate::FileReader;
use std::num::ParseIntError;
use std::ops::AddAssign;

const API_PATH: &str = "/api";

const HELLO: &str = "/hello";
const COUNT: &str = "/count";

const NAME_PARAM: &str = "name";
const VALUE_PARAM: &str = "value";

/// Handles all the request directed to the API under /api.
/// This handle responds to the requests with JSONs.
pub struct ApiHandler {
    fs: Box<FileReader>,
    counter: Counter,
}

impl ApiHandler {
    pub fn new(fs: Box<FileReader>) -> Self {
        Self {
            fs,
            counter: Counter::new(),
        }
    }

    /// GET /api/* router
    fn get_router(&self, path: &str, request: &Request) -> Option<Response> {
        match path {
            HELLO => self.get_hello(request.query()),
            COUNT => ok_response(self.counter.parse()),
            _ => None,
        }
    }

    /// POST /api/* router
    fn post_router(&mut self, path: &str, request: &Request) -> Option<Response> {
        match path {
            COUNT => self.post_count(request.query()),
            _ => None,
        }
    }

    /// DELETE /api/* router
    fn delete_router(&mut self, path: &str, _: &Request) -> Option<Response> {
        match path {
            COUNT => {
                self.counter.reset();
                Some(Response::new(StatusCode::NoContent, None))
            }
            _ => None,
        }
    }

    /// Manages the /api/hello endpoint. By default it returns a Hello World! JSON.
    /// In case that a "query" name exists it returns a JSON with Hello <name>!
    fn get_hello(&self, queries: Option<&QueryMap>) -> Option<Response> {
        if let Some(qm) = queries {
            if let Some(name) = qm.get(NAME_PARAM) {
                return ok_response(to_json("message", &format!("Hello {}!!!", name)));
            }
        }
        ok_response(
            self.fs
                .read_file("helloworld.txt")
                .and_then(|s| to_json("message", &s)),
        )
    }

    /// Manages the POST /api/count call. By default adds one to the current count.
    /// If a valid integer is supplied in the "value" query it adds that value.
    /// In case that the integer is valid a Bad Request response is thrown.
    fn post_count(&mut self, queries: Option<&QueryMap>) -> Option<Response> {
        let mut val = 1;
        if let Some(qm) = queries {
            if let Some(QueryValue::Single(v)) = qm.get(VALUE_PARAM) {
                match v.parse().map_err(|e: ParseIntError| e.into()) {
                    Ok(amount) => {
                        val = amount;
                    }
                    Err(e) => {
                        return Some(self.handle_bad_request(e));
                    }
                }
            }
        }
        self.counter += val;
        ok_response(self.counter.parse())
    }
}

impl Handler for ApiHandler {
    fn handle_request(&mut self, request: &Request) -> Option<Response> {
        // Works only in the API path
        if request.path().starts_with(API_PATH) {
            let api_path = &request.path()[API_PATH.bytes().count()..];
            match request.method() {
                Method::Get => self.get_router(api_path, request),
                Method::Post => self.post_router(api_path, request),
                Method::Delete => self.delete_router(api_path, request),
                _ => None,
            }
        } else {
            None
        }
    }

    fn default_response(&self) -> Response {
        Response::new(
            StatusCode::NotFound,
            to_json("message", StatusCode::NotFound.message()),
        )
    }
}

/// Builds a 200 Response with the given body
fn ok_response(content: Option<String>) -> Option<Response> {
    let mut response = Response::new(StatusCode::Ok, content);
    response.header(Header::ContentType, "application/json");
    Some(response)
}

/// Formats a basic key-value json
fn to_json(key: &str, value: &str) -> Option<String> {
    Some(format!("{{\"{}\":\"{}\"}}", key, value))
}

/// Abstraction of the Counter managed with the /api/count requests.
/// This implementation features operator overloading
struct Counter(i32);

impl Counter {
    fn new() -> Self {
        Self(0)
    }

    /// Resets the count to 0
    fn reset(&mut self) {
        self.0 = 0;
    }

    /// Returns the JSON body string of the current state of the counter
    fn parse(&self) -> Option<String> {
        Some(format!("{{\"count\":{}}}", self.0))
    }
}

/** += **/
impl AddAssign<i32> for Counter {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs
    }
}
