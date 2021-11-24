use super::StatusCode;

#[derive(Debug)]
pub struct Response {
	status_code: StatusCode,
	body: Option<String>,
}

impl Response {
	pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
		Self { status_code, body }
	}
}