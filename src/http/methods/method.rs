use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

pub enum Method {
	GET,
	DELETE,
	POST,
	PUT,
	HEAD,
	CONNECT,
	OPTIONS,
	TRACE,
	PATCH
}

/** Parsing */

impl FromStr for Method {
	type Err = InvalidMethodError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"GET" => Ok(Self::GET),
			"DELETE" => Ok(Self::DELETE),
			"POST" => Ok(Self::POST),
			"PUT" => Ok(Self::PUT),
			"HEAD" => Ok(Self::HEAD),
			"CONNECT" => Ok(Self::CONNECT),
			"OPTIONS" => Ok(Self::OPTIONS),
			"TRACE" => Ok(Self::TRACE),
			"PATCH" => Ok(Self::PATCH),
			_ => Err(InvalidMethodError)
		}
	}
}

pub struct InvalidMethodError;
