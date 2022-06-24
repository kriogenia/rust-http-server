use std::fs;

/// Reads and parses the files stored in the public folder
pub struct FileReader {
    public_path: String,
}

impl FileReader {
    pub fn new(public_path: &str) -> FileReader {
        FileReader {
            public_path: String::from(public_path),
        }
    }

    /// Tries to read the specified file and returns the content if it exists and is parseable
    /// It evades Directory Traversal Attacks
    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    eprintln!("Directory Traversal Attack attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}
