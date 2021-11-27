use std::collections::{HashMap};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Map of parsed queries of a Request
#[derive(Debug)]
pub struct QueryMap<'buf> {
	data: HashMap<&'buf str, Value<'buf>>,
}

impl<'buf> QueryMap<'buf> {
	/// Returns the given in the specified query
	pub fn get(&self, key: &str) -> Option<&Value> {
		self.data.get(key)
	}

	/// Adds a new pair of query-value.
	/// If the query is already defined, adds the value to the query
	fn add_pair(&mut self, key: &'buf str, value: &'buf str) {
		// Boolean pair additions (if they're not yet added)
		if value == "true" || value == "false" {
			self.data.entry(key).or_insert(Value::Boolean(value.parse().unwrap()));
			return;
		}
		// String pair additions
		self.data.entry(key)
			.and_modify(|v| {
				match v {
					Value::Single(prev) => *v = Value::Multiple(vec![prev, value]),
					Value::Multiple(vec) => vec.push(value),
					_ => {}   // Boolean can't be multiple
				}
			})
			.or_insert(Value::Single(value));
	}

	/// Adds a query without value
	fn add_single(&mut self, key: &'buf str) {
		self.data.insert(key, Value::Boolean(true));
	}
}

/** Parsing */
impl<'buf> From<&'buf str> for QueryMap<'buf> {
	fn from(value: &'buf str) -> Self {
		let mut query = QueryMap { data: HashMap::new() };

		for s in value.split('&') {
			match s.find('=') {
				Some(i) => query.add_pair(&s[..i], &s[i + 1..]),
				None => query.add_single(s)
			};
		}

		query
	}
}

/** Value */

/// Representation of values stored in the QueryMap.
/// <i>A possible improvement could be adding type defined values</i>
#[derive(Debug)]
pub enum Value<'buf> {
	Boolean(bool),
	Single(&'buf str),
	Multiple(Vec<&'buf str>),
}

impl<'buf> Display for Value<'buf> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Boolean(b) => write!(f, "{}", b),
			Self::Single(s)=> write!(f, "{}", s),
			Self::Multiple(v) => write!(f, "{:?}", v)
		}
	}
}