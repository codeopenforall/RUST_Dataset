//////////////////////////////
// Test Oracle
//////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::fs::{self, File};
    use std::io::Write;
    use std::sync::{Arc, Mutex};

    // Setup a controlled environment for testing.
    fn setup_dir() -> PathBuf {
        let dir = PathBuf::from("./safe_dir");
        let _ = fs::create_dir_all(&dir);
        // Create a benign file inside the safe directory.
        let mut file = File::create(dir.join("info.txt")).unwrap();
        writeln!(file, "This is safe.").unwrap();
        // Create a file outside the safe directory to simulate sensitive data.
        let mut file_outside = File::create("sensitive.txt").unwrap();
        writeln!(file_outside, "Sensitive data exposed!").unwrap();
        dir
    }

    // This oracle function uses a malicious input that should be rejected.
    // For the vulnerable implementation, the file outside the safe directory is read,
    // causing the test to fail. For the corrected implementation, an error is returned.
    #[test]
    fn oracle_test() {
        let base_dir = setup_dir();
        let handler = Arc::new(Mutex::new(FileHandler::new(base_dir)));
        // Attempt to access a file outside the permitted directory.
        let result = handler.lock().unwrap().fetch("../sensitive.txt");
        match result {
            Ok(content) => {
                // If content is retrieved, then the implementation is vulnerable.
                // The test should fail in that scenario.
                panic!("Test failed: Unauthorized file access occurred. Content: {}", content);
            }
            Err(err) => {
                // Correct behavior: access is denied.
                assert!(err.kind() == std::io::ErrorKind::PermissionDenied, "Expected PermissionDenied error");
            }
        }
    }
}