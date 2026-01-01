#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    // Helper to set up the test environment with known files.
    fn setup_environment() -> (PathBuf, PathBuf) {
        // Create base directory and an outside file for testing.
        let base_dir = PathBuf::from("data");
        let outside_path = PathBuf::from("secret.txt");

        // Ensure base directory exists.
        fs::create_dir_all(&base_dir).unwrap();
        // Create a dummy file inside base.
        let mut f_inside = fs::File::create(base_dir.join("inside.txt")).unwrap();
        writeln!(f_inside, "valid content").unwrap();
        // Create a dummy secret file outside of the base.
        let mut f_outside = fs::File::create(&outside_path).unwrap();
        writeln!(f_outside, "sensitive content").unwrap();

        (base_dir, outside_path)
    }

    // Test function that ensures the provided input does not allow escaping the directory.
    // For the vulnerable version, this test should fail because "../secret.txt" would be accepted.
    // For the corrected version, this test should pass due to proper path validation.
    #[test]
    fn test_path_traversal_prevention() {
        // Set up test files.
        let (base_dir, _outside_path) = setup_environment();
        let loader = Loader { base: base_dir };

        // Attempt to load a file outside the allowed directory.
        let result = loader.load("../secret.txt");
        assert!(result.is_err(), "Path traversal test failed: access outside base directory was allowed");
    }
}