use std::fs;

pub struct FileReader<'s> {
	public_path: &'s str
}

impl<'s> FileReader<'s> {
	pub fn new(public_path: &str) -> FileReader {
		FileReader { public_path }
	}

	pub fn read_file(&self, file_path: &str) -> Option<String> {
		let path = format!("{}/{}", self.public_path, file_path);
		match fs::canonicalize(path) {
			Ok(path) => {
				if path.starts_with(self.public_path) {
					fs::read_to_string(path).ok()
				} else {
					eprintln!("Directory Traversal Attack attempted: {}", file_path);
					None
				}
			},
			Err(_) => None
		}
	}

}