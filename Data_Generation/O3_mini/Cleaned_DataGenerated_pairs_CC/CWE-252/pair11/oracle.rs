#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::fs;
    use tempfile::tempdir;
    #[test]
    fn test_error_handling() {
        let dir = tempdir().expect("Failed to create temp dir");
        let log_path = dir.path().join("data.log");
        fs::write(&log_path, "initial").expect("Failed to write to log");
        let mut perms = fs::metadata(&log_path).expect("Failed to get metadata").permissions();
        perms.set_readonly(true);
        fs::set_permissions(&log_path, perms).expect("Failed to set permissions");
        let output = Command::new(std::env::current_exe().unwrap())
            .current_dir(dir.path())
            .output()
            .expect("Failed to run binary");
        assert!(!output.status.success(), "Process should fail on file write error");
    }
}
