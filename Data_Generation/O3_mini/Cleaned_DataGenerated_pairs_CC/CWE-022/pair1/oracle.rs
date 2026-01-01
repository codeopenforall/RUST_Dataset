#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::fs::{self, File};
    use std::io::Write;
    use std::sync::{Arc, Mutex};
    fn setup_dir() -> PathBuf {
        let dir = PathBuf::from("./safe_dir");
        let _ = fs::create_dir_all(&dir);
        let mut file = File::create(dir.join("info.txt")).unwrap();
        writeln!(file, "This is safe.").unwrap();
        let mut file_outside = File::create("sensitive.txt").unwrap();
        writeln!(file_outside, "Sensitive data exposed!").unwrap();
        dir
    }
    #[test]
    fn oracle_test() {
        let base_dir = setup_dir();
        let handler = Arc::new(Mutex::new(FileHandler::new(base_dir)));
        let result = handler.lock().unwrap().fetch("../sensitive.txt");
        match result {
            Ok(content) => {
                panic!("Test failed: Unauthorized file access occurred. Content: {}", content);
            }
            Err(err) => {
                assert!(err.kind() == std::io::ErrorKind::PermissionDenied, "Expected PermissionDenied error");
            }
        }
    }
}
