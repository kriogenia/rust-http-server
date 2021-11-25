use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
	Ok = 200,
	BadRequest = 400,
	_NotFound = 404,
	RequestTimeout = 408,
	_InternalServerError = 500
}

impl StatusCode {
	pub fn to_string(&self) -> &str {
		match self {
			Self::Ok => "Ok",
			Self::BadRequest => "Bad Request",
			Self::_NotFound => "Not Found",
			Self::RequestTimeout => "Request Timeout",
			Self::_InternalServerError => "Internal Server Error"
		}
	}
}

/** Printing */

impl Display for StatusCode {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", *self as u16)
	}
}