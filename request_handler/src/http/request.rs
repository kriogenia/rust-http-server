use crate::http::method::Method;
use std::convert::TryFrom;
use std::str;

use self::parsing_error::ParsingError;
use self::query::QueryMap;

pub mod query;
pub mod parsing_error;

/// Parsed HTTP Request
#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query: Option<QueryMap<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    /// Returns the request path
    pub fn path(&self) -> &'buf str {
        self.path
    }

    /// Returns the queries provided in the request
    pub fn query(&self) -> Option<&QueryMap<'buf>> {
        self.query.as_ref()
    }

    /// Returns the method of the query
    pub fn method(&self) -> &Method {
        &self.method
    }
}

/** Parsing **/

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParsingError;

    fn try_from(value: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(value)?;

        let (method, request) = get_next_word(request).ok_or(ParsingError::Request)?;
        let (reference, request) = get_next_word(request).ok_or(ParsingError::Request)?;
        let (protocol, _) = get_next_word(request).ok_or(ParsingError::Request)?;
        // ignoring the rest of the request for now

        if protocol != "HTTP/1.1" {
            return Err(ParsingError::Protocol);
        }

        let method: Method = method.parse()?;
        let (path, query) = read_reference(reference);

        Ok(Self {
            path,
            query,
            method,
        })
    }
}

/** Auxiliary methods */

/// Returns the next word in the buffer
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c.is_whitespace() {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

/// If the relative reference is defined, reads the queries and return the path and the query map
fn read_reference(reference: &str) -> (&str, Option<QueryMap>) {
    match reference.find('?') {
        Some(i) => (&reference[..i], Some(QueryMap::from(&reference[i + 1..]))),
        None => (reference, None),
    }
}
