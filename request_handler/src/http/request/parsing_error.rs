use crate::http::method::InvalidMethodError;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;

/// Represents the possible errors occurring during the parsing of a Request
pub enum ParsingError {
    Request,
    Encoding,
    Protocol,
    Method,
}

impl<'e> ParsingError {
    /// String message of the error
    pub fn message(&self) -> &'e str {
        use ParsingError::*;
        match self {
            Request => "Invalid request",
            Encoding => "Invalid encoding",
            Protocol => "Invalid protocol",
            Method => "Invalid method",
        }
    }
}

/** Error **/

impl Error for ParsingError {}

/**  Printing */

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

/** Conversions **/

impl From<Utf8Error> for ParsingError {
    fn from(_: Utf8Error) -> Self {
        Self::Encoding
    }
}

impl From<InvalidMethodError> for ParsingError {
    fn from(_: InvalidMethodError) -> Self {
        Self::Method
    }
}
