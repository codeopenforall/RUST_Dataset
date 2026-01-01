#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::MetadataExt;
    
    // The test oracle creates a resource file using the library API,
    // then verifies that the file permissions have been set to a secure value (0o600).
    // For the insecure implementation, the file mode would be 0o777 causing this test to fail.
    #[test]
    fn verify_file_permissions() {
        let instance = App::new();
        let test_path = "/tmp/test_resource.txt";
        let _ = fs::remove_file(test_path); // Clean up before testing
        
        instance.process(test_path);
        
        let metadata = fs::metadata(test_path).expect("Unable to retrieve metadata");
        let mode = metadata.mode() & 0o777; // Extract permission bits
        
        // Assert that the file mode is the secure mode (0o600).
        assert_eq!(mode, 0o600, "File permission is insecure: expected 0o600, found {:#o}", mode);
        let _ = fs::remove_file(test_path); // Clean up after test
    }
}