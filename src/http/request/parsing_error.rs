use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;
use crate::http::methods::method::InvalidMethodError;

pub enum ParsingError {
	InvalidRequest,
	InvalidEncoding,
	InvalidProtocol,
	InvalidMethod,
}

impl ParsingError {
	fn message(&self) -> &str {
		match self {
			Self::InvalidRequest => "Invalid request",
			Self::InvalidEncoding => "Invalid encoding",
			Self::InvalidProtocol => "Invalid protocol",
			Self::InvalidMethod => "Invalid method"
		}
	}
}

/** Error interface **/

impl Display for ParsingError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

impl Debug for ParsingError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

impl Error for ParsingError {}

/** Error conversions **/

impl From<Utf8Error> for ParsingError {
	fn from(_: Utf8Error) -> Self {
		Self::InvalidEncoding
	}
}
impl From<InvalidMethodError> for ParsingError {
	fn from(_: InvalidMethodError) -> Self { Self::InvalidMethod }
}