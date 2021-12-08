use crate::http::HttpError;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// HTTP Status Codes used in the Server
#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    NoContent = 204,
    BadRequest = 400,
    NotFound = 404,
    RequestTimeout = 408,
    _InternalServerError = 500,
}

impl StatusCode {
    /// String representation of the Code
    pub fn message(&self) -> &str {
        use StatusCode::*;
        match self {
            Ok => "Ok",
            NoContent => "No Content",
            BadRequest => "Bad Request",
            NotFound => "Not Found",
            RequestTimeout => "Request Timeout",
            _InternalServerError => "Internal Server Error",
        }
    }
}

/** Casting **/

impl From<HttpError<'_>> for StatusCode {
    fn from(e: HttpError) -> Self {
        match e {
            HttpError::BadRequest(_) => Self::BadRequest,
            HttpError::RequestTimeout => Self::RequestTimeout,
        }
    }
}

/** Printing **/

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
