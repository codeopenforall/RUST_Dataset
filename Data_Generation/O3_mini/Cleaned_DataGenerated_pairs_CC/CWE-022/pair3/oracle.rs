#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::io::ErrorKind;
    use super::*;
    #[test]
    fn test_secure_and_insecure_behavior() {
        let temp_dir = env::temp_dir().join("rust_test");
        let _ = fs::create_dir_all(&temp_dir);
        let allowed_file = temp_dir.join("allowed.txt");
        {
            let mut f = File::create(&allowed_file)
                .expect("failed to create allowed.txt");
            writeln!(f, "Safe Content").expect("failed to write to allowed.txt");
        }
        let handler = FileHandler::new(temp_dir.to_str().unwrap());
        let traversal_input = "../etc/passwd";
        let result = handler.load(traversal_input);
        match result {
            Ok(_) => panic!("Security test failed: Traversal input succeeded unexpectedly"),
            Err(e) => {
                assert_eq!(e.kind(), ErrorKind::InvalidInput, "Expected InvalidInput error");
            }
        }
        let result_valid = handler.load("allowed.txt");
        match result_valid {
            Ok(content) => assert!(content.contains("Safe Content"), "File content does not match expected"),
            Err(_) => panic!("Security test failed: Valid input was rejected")
        }
    }
}
