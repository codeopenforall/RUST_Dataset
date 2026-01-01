#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::fs;
    use tempfile::tempdir;

    // The test sets up a condition that forces file write failures
    // by creating a log file with read-only permissions. The correct
    // behavior is for the fixed version to propagate the error (non-zero exit),
    // while the vulnerable version (ignoring errors) would exit successfully.
    #[test]
    fn test_error_handling() {
        // Create a temporary directory.
        let dir = tempdir().expect("Failed to create temp dir");
        let log_path = dir.path().join("data.log");

        // Create a log file and write initial content.
        fs::write(&log_path, "initial").expect("Failed to write to log");

        // Set the file to read-only mode.
        let mut perms = fs::metadata(&log_path).expect("Failed to get metadata").permissions();
        perms.set_readonly(true);
        fs::set_permissions(&log_path, perms).expect("Failed to set permissions");

        // Invoke the compiled binary in the temp directory.
        let output = Command::new(std::env::current_exe().unwrap())
            .current_dir(dir.path())
            .output()
            .expect("Failed to run binary");

        // Correct implementation should detect the write error and exit with a failure.
        // Therefore, a non-success exit status indicates the error was properly handled.
        assert!(!output.status.success(), "Process should fail on file write error");
    }
}