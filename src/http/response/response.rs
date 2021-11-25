use std::io::{Result as IoResult, Write};
use super::StatusCode;

const HTTP_HEADER: &str = "HTTP/1.1";

#[derive(Debug)]
pub struct Response {
	status_code: StatusCode,
	body: Option<String>,
}

impl Response {
	pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
		Self { status_code, body }
	}

	pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
		let body = if self.body.is_some() { &self.body.as_ref().unwrap() } else { "" };
		write!(
			stream,
			"{} {} {}\r\n\r\n{}",
			HTTP_HEADER,
			self.status_code,
			self.status_code.to_string(),
			body)
	}
}