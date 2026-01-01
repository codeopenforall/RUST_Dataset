#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn test_path_traversal() {
        let safe = Path::new("safe_dir");
        let manager = Manager::new(safe);
        let malicious_input = "../escape.txt";
        let result = manager.get_path(malicious_input);
        assert!(
            result.is_err(),
            "Path traversal input should be rejected, but it was accepted."
        );
    }
}
