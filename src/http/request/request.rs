use super::ParsingError;
use crate::http::Method;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str;

pub struct Request {
	path: String,
	query: Option<String>,
	method: Method,
}

/** Parsing **/

impl TryFrom<&[u8]> for Request {
	type Error = ParsingError;

	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
		let request = str::from_utf8(value)?;

		let (method, request) = get_next_word(request).ok_or(ParsingError::InvalidRequest)?;
		let (reference, request) = get_next_word(request).ok_or(ParsingError::InvalidRequest)?;
		let (protocol, _) = get_next_word(request).ok_or(ParsingError::InvalidRequest)?;
		// ignoring the rest of the request for now

		if protocol != "HTTP/1.1" {
			return Err(ParsingError::InvalidProtocol);
		}

		let method: Method = method.parse()?;
		let (path, query) = read_reference(reference);

		println!("Path: {}", path);
		if query.is_some() { println!("Query: {}", query.unwrap()); }
		println!("Protocol: {}", protocol);

		unimplemented!()
	}
}

/** Printing */

impl Display for Request {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		let query = if self.query.is_some() { self.query.as_ref().unwrap() } else { "-" };
		write!(f,
		       "REQUEST:\
		       Path: {}\
		       Query: {}",
		       self.path, query)
	}
}

/** Auxiliary methods */

fn get_next_word(request: &str) -> Option<(&str, &str)> {
	for (i, c) in request.chars().enumerate() {
		if c.is_whitespace() {
			return Some((&request[..i], &request[i + 1..]));
		}
	}
	return None;
}

fn read_reference(reference: &str) -> (&str, Option<&str>) {
	match reference.find('?') {
		Some(i) => (&reference[..i], Some(&reference[i + 1..])),
		None => (&reference, None)
	}
}