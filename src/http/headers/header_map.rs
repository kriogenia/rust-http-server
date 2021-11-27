use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use super::Header;

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