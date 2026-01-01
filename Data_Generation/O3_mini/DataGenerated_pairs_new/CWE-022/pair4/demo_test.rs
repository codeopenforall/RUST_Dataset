//////////////////////////////////////////////
// Test Oracle for CWE-022 Path Traversal
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, create_dir_all, write};
    use std::io::Write;
    use std::path::PathBuf;

    // Helper to set up a simulated file environment.
    fn setup_test_files(base: &str) -> (PathBuf, PathBuf) {
        let base_path = PathBuf::from(base);
        create_dir_all(&base_path).expect("Failed to create base directory");
        // Create a safe file inside the allowed directory.
        let safe_file = base_path.join("safe.txt");
        write(&safe_file, "Allowed content").expect("Failed to write safe file");
        // Create a sensitive file outside the allowed directory.
        let secret_file = PathBuf::from("./secret.txt");
        write(&secret_file, "Sensitive information").expect("Failed to write secret file");
        (base_path, secret_file)
    }

    #[test]
    fn test_path_traversal() {
        // Set up a temporary environment.
        let base_dir = "./data";
        let (_allowed_dir, _secret_file) = setup_test_files(base_dir);

        // Prepare a malicious input attempting directory traversal.
        let malicious = "../secret.txt";

        // Instantiate the file handler.
        let handler = FileHandler::new(base_dir);
        let result = handler.read_file(malicious);

        // The secure implementation should reject the path traversal attempt.
        assert!(result.is_err(), "Path traversal check failed: access outside allowed directory.");
    }
}