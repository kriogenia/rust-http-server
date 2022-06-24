use std::str::FromStr;

/// HTTP Methods
#[derive(Debug)]
pub enum Method {
    Get,
    Delete,
    Post,
    Put,
    Head,
    Connect,
    Options,
    Trace,
    Patch,
}

/** Parsing */

impl FromStr for Method {
    type Err = InvalidMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Method::*;
        match s {
            "GET" => Ok(Get),
            "DELETE" => Ok(Delete),
            "POST" => Ok(Post),
            "PUT" => Ok(Put),
            "HEAD" => Ok(Head),
            "CONNECT" => Ok(Connect),
            "OPTIONS" => Ok(Options),
            "TRACE" => Ok(Trace),
            "PATCH" => Ok(Patch),
            _ => Err(InvalidMethodError),
        }
    }
}

pub struct InvalidMethodError;
