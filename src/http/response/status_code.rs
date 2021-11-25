use std::fmt::{Display, Formatter};
use crate::http::HttpError;

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
	Ok = 200,
	BadRequest = 400,
	NotFound = 404,
	RequestTimeout = 408,
	_InternalServerError = 500
}

impl StatusCode {
	pub fn to_string(&self) -> &str {
		match self {
			Self::Ok => "Ok",
			Self::BadRequest => "Bad Request",
			Self::NotFound => "Not Found",
			Self::RequestTimeout => "Request Timeout",
			Self::_InternalServerError => "Internal Server Error"
		}
	}
}

// Casting

impl From<HttpError<'_>> for StatusCode {
	fn from(e: HttpError) -> Self {
		match e {
			HttpError::BadRequest(_) => Self::BadRequest,
			HttpError::RequestTimeout => Self::RequestTimeout
		}
	}
}

// Printing

impl Display for StatusCode {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", *self as u16)
	}
}