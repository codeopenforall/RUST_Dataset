#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::os::unix::fs::symlink;
    use std::path::PathBuf;
    use std::sync::Arc;

    // Test to validate that the file access function properly restricts symbolic link resolution.
    // For the corrected version, trying to access a symlink that points outside the allowed directory should fail.
    #[test]
    fn test_symlink_restriction() {
        // Create a temporary base directory.
        let base_dir = PathBuf::from("/tmp/test_secure_dir");
        let external_dir = PathBuf::from("/tmp/test_external");
        let _ = fs::create_dir_all(&base_dir);
        let _ = fs::create_dir_all(&external_dir);

        // Create an external file that should not be accessible.
        let forbidden_file = external_dir.join("forbidden.txt");
        let mut out_file = File::create(&forbidden_file).expect("Unable to create external file");
        writeln!(out_file, "Sensitive data").expect("Unable to write to external file");

        // Create a symlink inside base_dir that points to the forbidden file.
        let symlink_path = base_dir.join("link.txt");
        let _ = fs::remove_file(&symlink_path);
        symlink(&forbidden_file, &symlink_path).expect("Failed to create symlink");

        let file_handler = Arc::new(FileHandler::new(base_dir));
        // Attempt to access the symlink.
        let result = file_handler.access("link.txt");

        // The corrected implementation MUST reject the symlink resolution.
        // For the vulnerable implementation, this test would erroneously succeed.
        assert!(result.is_err(), "Symlink resolution was not properly restricted");
    }
}