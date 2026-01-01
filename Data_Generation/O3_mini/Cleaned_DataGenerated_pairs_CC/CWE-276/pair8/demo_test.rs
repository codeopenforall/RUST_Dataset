#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::MetadataExt;

    #[test]
    fn test_file_permissions() {
        // Clean up any pre-existing file.
        let _ = fs::remove_file("service.log");

        // Invoke the application's main function. This test applies to both implementations.
        // For the insecure variant, the file will have mode 0o777, causing the test to fail.
        crate::main();

        let metadata = fs::metadata("service.log").expect("service.log was not created");
        // Extract the permission bits.
        let mode = metadata.mode() & 0o777;
        // The secure version should create the file with mode 0o600.
        assert_eq!(mode, 0o600, "File permissions are not set securely: got {:o}", mode);

        // Clean up after test.
        let _ = fs::remove_file("service.log");
    }
}