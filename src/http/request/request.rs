use super::ParsingError;
use crate::http::Method;
use std::convert::TryFrom;
use std::str;

pub struct Request {
	path: String,
	query_string: Option<String>,
	method: Method,
}

impl TryFrom<&[u8]> for Request {
	type Error = ParsingError;

	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
		let request = str::from_utf8(value)?;

		let (method, request) = get_next_word(request).ok_or(ParsingError::InvalidRequest)?;
		let (path, request) = get_next_word(request).ok_or(ParsingError::InvalidRequest)?;
		let (protocol, _) = get_next_word(request).ok_or(ParsingError::InvalidRequest)?;

		if protocol != "HTTP/1.1" {
			return Err(ParsingError::InvalidProtocol);
		}

		println!("Method: {}", method);
		println!("Path: {}", path);
		println!("Protocol: {}", protocol);

		unimplemented!();
	}
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
	for (i, c) in request.chars().enumerate() {
		if c == ' ' || c == '\r' || c == '\n'  {
			return Some((&request[..i], &request[i + 1..]));
		}
	}
	return None;
}