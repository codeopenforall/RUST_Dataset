#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    #[test]
    fn test_file_access() {
        let manager = Arc::new(Manager::new("/safe_dir/"));
        let malicious = manager.open("../etc/passwd");
        assert!(malicious.is_err(), "Traversal input should be denied.");
        let safe = manager.open("file.txt");
        assert!(safe.is_ok(), "Safe access should succeed.");
        assert_eq!(safe.unwrap(), "regular_data");
    }
}
