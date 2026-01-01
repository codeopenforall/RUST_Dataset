#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{write, create_dir_all, remove_dir_all};
    use std::path::PathBuf;

    // The test oracle verifies that attempts at path traversal are blocked.
    // For the vulnerable code, the traversal attempt returns file content; for the fixed code, it returns None.
    #[test]
    fn test_path_traversal() {
        // Setup a temporary safe directory.
        let base = PathBuf::from("./temp_safe_dir");
        let allowed_file = base.join("allowed.txt");
        // Create a file outside the allowed directory.
        let outside_file = base.parent().unwrap().join("outside.txt");

        // Create directories and files for testing.
        create_dir_all(&base).unwrap();
        write(&allowed_file, b"allowed content").unwrap();
        write(&outside_file, b"outside content").unwrap();

        let handler = ResourceHandler::new(base.clone());

        // Verify that a file within the base directory is read correctly.
        let safe_content = handler.load("allowed.txt");
        assert_eq!(safe_content.unwrap(), "allowed content");

        // Attempt path traversal using a relative path that escapes the base directory.
        let traversal_path = "../outside.txt";
        let traversal_content = handler.load(traversal_path);
        // The fixed version should block path traversal and return None.
        assert!(
            traversal_content.is_none(),
            "Traversal attack should be blocked, but file was accessed."
        );

        // Clean up the test files and directories.
        remove_dir_all(&base).unwrap();
        let _ = std::fs::remove_file(outside_file);
    }
}