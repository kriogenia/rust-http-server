use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult, write};

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
