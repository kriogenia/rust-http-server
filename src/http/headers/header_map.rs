use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use super::Header;

/// Request/Response Headers map
/// <i>Improvable with an approach similar to QueryMap allowing a better way to store lists</i>
#[derive(Debug)]
pub struct HeaderMap {
	data: HashMap<Header, String>,
}

impl HeaderMap {
	pub fn new() -> Self {
		HeaderMap {
			data: HashMap::new()
		}
	}

	/// Adds a new header and its value to the current list
	/// <i>Right now, there's no current way to remove as it wasn't need in this implementation*>
	pub fn add(&mut self, key: Header, value: &str) {
		self.data.insert(key, value.to_string());
	}
}

impl Display for HeaderMap {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		for (k, v) in self.data.iter() {
			write!(f, "{}: {}\n", k, v)?;
		}
		write!(f, "")
	}
}