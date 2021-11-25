use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use crate::http::request::ParsingError;
use crate::http::StatusCode;

pub enum HttpError {
	BadRequest(String),
	RequestTimeout,
}

impl HttpError {
	pub fn message(&self) -> &str {
		match self {
			Self::BadRequest(message) => message,
			Self::RequestTimeout => "Request timeout"
		}
	}

	pub fn code(&self) -> StatusCode {
		match self {
			Self::BadRequest(_) => StatusCode::BadRequest,
			Self::RequestTimeout => StatusCode::RequestTimeout
		}
	}

	/*
	pub fn toJson(&self) -> &str {
		let mut message = "{\"message\":\"".to_owned();
		message.push_str(self.message());
		message.push_str("\"}")
	}*/
}

/** Error */

impl Error for HttpError {}

/** Printing */

impl Debug for HttpError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

impl<'e> Display for HttpError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

/** Conversions */

impl From<ParsingError> for HttpError {
	fn from(e: ParsingError) -> Self {
		Self::BadRequest(e.message().to_owned())
	}
}