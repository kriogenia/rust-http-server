use std::fmt::{Display, Formatter, write};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
	Ok = 200,
	BadRequest = 400,
	NotFound = 404,
	InternalServerError = 500
}

impl StatusCode {
	pub fn to_string(&self) -> &str {
		match self {
			Self::Ok => "OK",
			Self::BadRequest => "BAD REQUEST",
			Self::NotFound => "NOT FOUND",
			Self::InternalServerError => "INTERNAL SERVER ERROR"
		}
	}
}

/** Printing */

impl Display for StatusCode {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", *self as u16)
	}
}