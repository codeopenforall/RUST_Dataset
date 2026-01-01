#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;
    use std::os::unix::fs::symlink;
    use std::path::Path;
    use std::env;
    use std::process::Command;

    // IMPORTANT: This test expects that the implementation under test provides a public structure named
    // FileResolver with a method new(PathBuf) -> FileResolver and a method load(&self, &Path) -> Result<String, String>.
    // When testing the implementation with the vulnerability, the test should fail by successfully reading the
    // contents of a symlinked file. When testing the secure implementation, the test should pass by rejecting the symlink.
    use crate::FileResolver;

    #[test]
    fn test_symlink_resolution() {
        // Setup: Create a controlled environment under /tmp/allowed_test.
        let allowed_dir = "/tmp/allowed_test";
        let dangerous_file = "/tmp/dangerous_test.txt";
        let test_file = "test.txt";
        let allowed_path = Path::new(allowed_dir);

        // Create allowed directory.
        fs::create_dir_all(allowed_path).expect("Failed to create allowed directory");
        // Write the dangerous file.
        let mut dangerous = File::create(dangerous_file).expect("Failed to create dangerous file");
        dangerous.write_all(b"DANGEROUS").expect("Write failed");

        // Remove any pre-existing test file.
        let test_path = allowed_path.join(test_file);
        let _ = fs::remove_file(&test_path);
        // Create a symlink inside the allowed directory that points to the dangerous file.
        symlink(dangerous_file, &test_path).expect("Symlink creation failed");

        let resolver = FileResolver::new(allowed_path.to_path_buf());
        let result = resolver.load(Path::new(test_file));

        // The secure implementation should reject the symlink by returning an error.
        match result {
            Ok(content) => {
                panic!("Test failed: expected an error but got content: {}", content);
            }
            Err(e) => {
                assert!(e.contains("Symlink escapes allowed directory"), "Unexpected error: {}", e);
            }
        }
    }
}