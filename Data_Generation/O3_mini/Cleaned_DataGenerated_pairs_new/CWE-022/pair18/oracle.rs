#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;
    use super::*;
    #[test]
    fn test_path_traversal_restriction() {
        let temp_dir = env::temp_dir().join("safe_dir_test");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).expect("Failed to create safe directory");
        let valid_path = temp_dir.join("valid.txt");
        let mut valid_file = File::create(&valid_path).expect("Failed to create valid file");
        writeln!(valid_file, "This is valid content").expect("Failed to write valid file");
        let secret_path = temp_dir.parent().unwrap().join("secret.txt");
        let mut secret_file = File::create(&secret_path).expect("Failed to create secret file");
        writeln!(secret_file, "Sensitive data").expect("Failed to write secret file");
        let handler = FileHandler::new(temp_dir.to_str().unwrap());
        let access_valid = handler.load("valid.txt");
        assert!(access_valid.is_ok(), "Expected valid file access");
        let traversal_attempt = handler.load("../secret.txt");
        assert!(
            traversal_attempt.is_err(),
            "Path traversal should be rejected"
        );
        let _ = fs::remove_file(secret_path);
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
