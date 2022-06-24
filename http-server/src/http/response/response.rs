use super::StatusCode;
use crate::http::headers::HeaderMap;
use crate::http::Header;
use std::io::{Result as IoResult, Write};

const HTTP_HEADER: &str = "HTTP/1.1";

/// Response to return to the client request
#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    headers: HeaderMap,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self {
            status_code,
            headers: HeaderMap::new(),
            body,
        }
    }

    /// Sets a new header for the Response
    pub fn header(&mut self, key: Header, value: &str) {
        self.headers.add(key, value);
    }

    /// Sends the response throught the given stream
    pub fn send(&mut self, stream: &mut impl Write) -> IoResult<()> {
        self.headers.add(Header::ContentLanguage, "en-UK");

        let body = if self.body.is_some() {
            &self.body.as_ref().unwrap()
        } else {
            ""
        };
        self.headers
            .add(Header::ContentLength, &format!("{}", body.chars().count()));

        println!("< {:?}", self);
        write!(
	        stream,
	        "{} {} {}\n\
			{}\n\
			{}",
	        HTTP_HEADER,
	        self.status_code,
	        self.status_code.message(),
	        self.headers,
	        body
        )
    }
}
