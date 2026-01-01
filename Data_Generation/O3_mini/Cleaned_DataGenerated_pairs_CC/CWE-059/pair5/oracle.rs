#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs as unix_fs;
    #[test]
    fn test_symlink_restriction() {
        let real_file = "real_output.txt";
        fs::write(real_file, "Initial data").expect("Failed to create the real file");
        let symlink_path = "test_symlink.txt";
        let _ = fs::remove_file(symlink_path);
        unix_fs::symlink(real_file, symlink_path).expect("Failed to create symlink");
        let handler = FileHandler {
            path: symlink_path.to_string(),
        };
        let result = handler.process("Malicious input\n");
        assert!(result.is_err(), "Symbolic link check failed: symlink processing should be rejected");
        let _ = fs::remove_file(symlink_path);
        let _ = fs::remove_file(real_file);
    }
}
