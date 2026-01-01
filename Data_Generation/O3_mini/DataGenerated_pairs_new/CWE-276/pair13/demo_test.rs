#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    // Oracle function: it verifies that the file permissions are secure (0o600).
    // When run against the insecure version, the test will fail because the file has mode 0o777.
    #[test]
    fn oracle_test() {
        // Ensure a clean state.
        let _ = fs::remove_file("data.txt");
        let manager = FileManager::new("data.txt");
        manager.generate().expect("File generation failed");
        
        let metadata = fs::metadata("data.txt").expect("Failed to retrieve metadata");
        let mode = metadata.permissions().mode() & 0o777;
        // Expect secure file permissions (read and write for owner only)
        assert_eq!(mode, 0o600, "File permissions are not secure: found {:o}", mode);
    }
}