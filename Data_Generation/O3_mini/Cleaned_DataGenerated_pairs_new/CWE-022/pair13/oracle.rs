#[cfg(test)]
mod tests {
    use std::io::ErrorKind;
    use super::FileHandler;
    #[test]
    fn test_path_traversal() {
        let handler = FileHandler::new("safe_dir".to_string());
        let result = handler.get("../secret.txt");
        assert!(result.is_err(), "Path traversal not prevented");
        if let Err(e) = result {
            assert_eq!(e.kind(), ErrorKind::PermissionDenied, "Unexpected error kind");
        }
    }
}
