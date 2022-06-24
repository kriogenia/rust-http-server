use crate::http::request::ParsingError;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::num::ParseIntError;

/// HTTP Errors convertible to 4xx Responses
pub enum HttpError<'e> {
    BadRequest(&'e str),
    RequestTimeout,
}

impl<'e> HttpError<'e> {
    /// Informative Error message
    pub fn message(&self) -> &str {
        use HttpError::*;
        match self {
            BadRequest(message) => message,
            RequestTimeout => "Request timeout",
        }
    }
}

/** Error */

impl<'e> Error for HttpError<'e> {}

/** Printing */

impl<'e> Debug for HttpError<'e> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl<'e> Display for HttpError<'e> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

/** Conversions */

impl<'e> From<ParsingError> for HttpError<'e> {
    fn from(e: ParsingError) -> Self {
        Self::BadRequest(e.message())
    }
}

impl<'e> From<ParseIntError> for HttpError<'e> {
    fn from(_: ParseIntError) -> Self {
        Self::BadRequest("The provided value is not a string")
    }
}
