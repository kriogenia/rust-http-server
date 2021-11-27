use std::num::ParseIntError;
use std::ops::AddAssign;
use crate::FileReader;
use crate::handlers::Handler;
use crate::http::{Header, Method, QueryMap, QueryValue, Request, Response, StatusCode};

const API_PATH: &str = "/api";

const HELLO: &str = "/hello";
const COUNT: &str = "/count";

const NAME_PARAM: &str = "name";
const VALUE_PARAM: &str = "value";

pub struct ApiHandler {
	fs: Box<FileReader>,
	counter: Counter,
}

impl ApiHandler {
	pub fn new(fs: Box<FileReader>) -> Self {
		Self { fs, counter: Counter::new() }
	}

	fn get_router(&self, path: &str, request: &Request) -> Option<Response> {
		match path {
			HELLO => self.get_hello(request.query()),
			COUNT => ok_response(self.counter.parse()),
			_ => None
		}
	}

	fn post_router(&mut self, path: &str, request: &Request) -> Option<Response> {
		match path {
			COUNT => self.post_count(request.query()),
			_ => None
		}
	}

	fn delete_router(&mut self, path: &str, _: &Request) -> Option<Response> {
		match path {
			COUNT => {
				self.counter.reset();
				Some(Response::new(StatusCode::NoContent, None))
			}
			_ => None
		}
	}

	fn get_hello(&self, queries: Option<&QueryMap>) -> Option<Response> {
		if queries.is_some() {
			if let Some(name) = queries.unwrap().get(NAME_PARAM) {
				return ok_response(to_json("message", &format!("Hello {}!!!", name)));
			}
		}
		ok_response(self.fs.read_file("helloworld.txt")
			.and_then(|s| to_json("message", &s)))
	}

	fn post_count(&mut self, queries: Option<&QueryMap>) -> Option<Response> {
		let mut val = 1;
		if queries.is_some() {
			if let Some(QueryValue::Single(v)) = queries.unwrap().get(VALUE_PARAM) {
				match v.parse().map_err(|e: ParseIntError| e.into()) {
					Ok(amount) => { val = amount; },
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
				Method::GET => self.get_router(api_path, request),
				Method::POST => self.post_router(api_path, request),
				Method::DELETE => self.delete_router(api_path, request),
				_ => None
			}
		} else { None }
	}

	fn default_response(&self) -> Response {
		Response::new(StatusCode::NotFound,
		              to_json("message", StatusCode::NotFound.to_string()))
	}
}

fn ok_response(content: Option<String>) -> Option<Response> {
	let mut response = Response::new(StatusCode::Ok, content);
	response.header(Header::ContentType, "application/json");
	Some(response)
}

fn to_json(key: &str, value: &str) -> Option<String> {
	Some(format!("{{\"{}\":\"{}\"}}", key, value))
}

/** Counter */

struct Counter(i32);

impl Counter {
	fn new() -> Self {
		Self(0)
	}

	fn reset(&mut self) {
		self.0 = 0;
	}

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