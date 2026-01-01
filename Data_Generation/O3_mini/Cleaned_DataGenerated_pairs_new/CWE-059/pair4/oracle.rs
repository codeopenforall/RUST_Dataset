#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::panic;
    use std::path::Path;
    use super::process;
    #[test]
    fn test_symlink_validation() {
        let external_file = "/tmp/test_external.txt";
        fs::write(external_file, b"secret data").expect("Failed to write external file");
        let symlink_path = "/tmp/malicious_link";
        let _ = fs::remove_file(symlink_path);
        symlink(external_file, symlink_path).expect("Failed to create symlink");
        let result = panic::catch_unwind(|| {
            process(symlink_path);
        });
        assert!(result.is_err(), "Security check did not trigger panic on malicious symlink");
        let _ = fs::remove_file(external_file);
        let _ = fs::remove_file(symlink_path);
    }
}
