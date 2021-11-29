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
        match s {
            "GET" => Ok(Self::Get),
            "DELETE" => Ok(Self::Delete),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "HEAD" => Ok(Self::Head),
            "CONNECT" => Ok(Self::Connect),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            "PATCH" => Ok(Self::Patch),
            _ => Err(InvalidMethodError),
        }
    }
}

pub struct InvalidMethodError;
