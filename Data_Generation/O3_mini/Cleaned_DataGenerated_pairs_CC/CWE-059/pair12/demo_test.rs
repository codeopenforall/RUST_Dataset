#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // This test oracle should be used separately against both versions.
    // For the vulnerable implementation, the test should fail because the malicious input returns "sensitive_data".
    // For the fixed implementation, the test should pass because the malicious input is rejected.
    #[test]
    fn test_file_access() {
        let manager = Arc::new(Manager::new("/safe_dir/"));
        // Simulated traversal input.
        let malicious = manager.open("../etc/passwd");
        // In the secure (fixed) version, traversal is denied.
        assert!(malicious.is_err(), "Traversal input should be denied.");

        // Test safe access.
        let safe = manager.open("file.txt");
        assert!(safe.is_ok(), "Safe access should succeed.");
        assert_eq!(safe.unwrap(), "regular_data");
    }
}