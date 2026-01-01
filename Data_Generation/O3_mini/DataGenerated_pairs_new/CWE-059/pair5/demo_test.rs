#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs as unix_fs;

    // This test attempts to use a symbolic link as the target file.
    // For the vulnerable version, the process call would erroneously succeed
    // and write data to the linked file, while the corrected version should reject it.
    #[test]
    fn test_symlink_restriction() {
        // Prepare a real file and a symlink pointing to it.
        let real_file = "real_output.txt";
        fs::write(real_file, "Initial data").expect("Failed to create the real file");

        let symlink_path = "test_symlink.txt";
        let _ = fs::remove_file(symlink_path);
        unix_fs::symlink(real_file, symlink_path).expect("Failed to create symlink");

        // Instantiate the handler with the symlink as the target.
        let handler = FileHandler {
            path: symlink_path.to_string(),
        };

        // Process should return an error if symbolic links are properly blocked.
        let result = handler.process("Malicious input\n");
        assert!(result.is_err(), "Symbolic link check failed: symlink processing should be rejected");

        // Cleanup.
        let _ = fs::remove_file(symlink_path);
        let _ = fs::remove_file(real_file);
    }
}