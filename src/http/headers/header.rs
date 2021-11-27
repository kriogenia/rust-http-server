use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Header {
	ContentLanguage,
	ContentLength
}

/** Printing **/

impl Header {
	fn to_string(&self) -> &str {
		match self {
			Self::ContentLanguage => "Content-Language",
			Self::ContentLength => "Content-Length"
		}
	}
}

impl Display for Header {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.to_string())
	}
}