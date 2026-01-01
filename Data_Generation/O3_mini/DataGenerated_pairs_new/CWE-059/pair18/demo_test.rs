/*
Test Oracle Function

This function serves as a test oracle to verify the protection against path traversal.
It attempts to access a file outside the allowed directory using a relative path such as "../Cargo.toml".
For the vulnerable code, this call would succeed (returning Ok with file content), causing the test to fail.
For the fixed code, the validation check should reject the attempt, returning an Err.
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    // Helper function to set up a dummy allowed directory with a safe file.
    fn setup_allowed() {
        let _ = fs::create_dir("allowed");
        let safe_file = "allowed/safe.txt";
        let mut file = File::create(safe_file).expect("Unable to create safe file");
        file.write_all(b"Safe Content").expect("Unable to write to safe file");
    }

    #[test]
    fn test_protection() {
        setup_allowed();

        let loader = Loader { base: "allowed/".to_string() };

        // Attempt to access a file using a path traversal payload.
        // In a secure implementation, this should be rejected.
        let result = loader.get_content("../Cargo.toml");
        assert!(result.is_err(), "Test failed: external file was accessed!");
    }
}