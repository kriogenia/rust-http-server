use super::ParsingError;
use crate::http::Method;
use std::convert::TryFrom;
use std::str;
use crate::http::request::ParsingError::InvalidEncoding;

pub struct Request {
	path: String,
	query_string: Option<String>,
	method: Method,
}

impl TryFrom<&[u8]> for Request {
	type Error = ParsingError;

	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
		let request = str::from_utf8(value)?;

	}

}