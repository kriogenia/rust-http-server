use std::collections::{HashMap};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct QueryMap<'buf> {
	data: HashMap<&'buf str, Value<'buf>>,
}

impl<'buf> QueryMap<'buf> {
	pub fn get(&self, key: &str) -> Option<&Value> {
		self.data.get(key)
	}

	fn add_pair(&mut self, key: &'buf str, value: &'buf str) {
		dbg!("{}: {}", key, value);
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