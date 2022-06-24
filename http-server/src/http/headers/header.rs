use std::fmt::{Display, Formatter, Result as FmtResult};

/// HTTP Headers currently supported in the server
/// <i>Currently, these are only directed to Responses.
/// This requires to implement TryFrom<u8> to be parsed from the buffer and used in Requests.</i>
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Header {
    _Authentication,
    ContentLanguage,
    ContentLength,
    ContentType,
}

/** Printing **/

impl Header {
    /// Returns the string representation of the Header
    fn to_string(&self) -> &str {
        use Header::*;
        match self {
            _Authentication => "Authentication",
            ContentLanguage => "Content-Language",
            ContentLength => "Content-Length",
            ContentType => "Content-Type",
        }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_string())
    }
}
