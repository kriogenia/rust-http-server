use super::{ParsingError, QueryMap};
use crate::http::Method;
use std::convert::TryFrom;
use std::str;

#[derive(Debug)]
pub struct Request<'buf> {
	path: &'buf str,
	query: Option<QueryMap<'buf>>,
	method: Method,
}

/** Parsing **/

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
	type Error = ParsingError;

	fn try_from(value: &'buf [u8]) -> Result<Self, Self::Error> {
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

		Ok(Self { path, query, method })
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

fn read_reference(reference: &str) -> (&str, Option<QueryMap>) {
	match reference.find('?') {
		Some(i) => (&reference[..i], Some(QueryMap::from(&reference[i + 1..]))),
		None => (&reference, None)
	}
}