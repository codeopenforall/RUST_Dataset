#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::MetadataExt;
    use std::process::Command;
    use std::path::Path;

    // This test oracle invokes the main function from the binary and then checks the created file's mode.
    // For a secure build, the file permissions must be 0o600, whereas the insecure version will leave it at 0o777.
    #[test]
    fn test_file_permissions() {
        // Remove any existing file
        let _ = fs::remove_file("insecure_log.txt");

        // Call the application's main function. It is assumed that the binary under test is already built.
        // This approach works in an integration test context.
        crate::main();

        // Confirm the file was created.
        let path = Path::new("insecure_log.txt");
        assert!(path.exists(), "Log file not created.");

        // Read file metadata to check permissions.
        let metadata = fs::metadata(path).expect("Unable to read metadata");
        let actual_mode = metadata.mode() & 0o777; // isolate permission bits

        // The fixed version should have mode 0o600.
        let expected_mode = 0o600;
        assert_eq!(
            actual_mode,
            expected_mode,
            "File permissions are insecure: expected {:o}, got {:o}",
            expected_mode,
            actual_mode
        );
    }
}