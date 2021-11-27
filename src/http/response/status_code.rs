use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::http::HttpError;

/// HTTP Status Codes used in the Server
#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
	Ok = 200,
	NoContent = 204,
	BadRequest = 400,
	NotFound = 404,
	RequestTimeout = 408,
	_InternalServerError = 500
}

impl StatusCode {
	/// String representation of the Code
	pub fn to_string(&self) -> &str {
		match self {
			Self::Ok => "Ok",
			Self::NoContent => "No Content",
			Self::BadRequest => "Bad Request",
			Self::NotFound => "Not Found",
			Self::RequestTimeout => "Request Timeout",
			Self::_InternalServerError => "Internal Server Error"
		}
	}
}

/** Casting **/

impl From<HttpError<'_>> for StatusCode {
	fn from(e: HttpError) -> Self {
		match e {
			HttpError::BadRequest(_) => Self::BadRequest,
			HttpError::RequestTimeout => Self::RequestTimeout
		}
	}
}

/** Printing **/

impl Display for StatusCode {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", *self as u16)
	}
}