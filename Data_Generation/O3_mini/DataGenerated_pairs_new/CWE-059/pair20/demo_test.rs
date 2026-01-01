////////////////////////////////////////////////////////////
// Test Oracle for the Symbolic Link Resolution Vulnerability
////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;
    use std::os::unix::fs::symlink;
    use super::*;

    #[test]
    fn test_symlink_restriction() {
        // Prepare a sensitive file outside the allowed directory.
        let sensitive_file = "sensitive.txt";
        let mut f = File::create(sensitive_file).expect("Failed to create sensitive file");
        writeln!(f, "secret content").expect("Failed to write sensitive file");
        
        // Create a symbolic link that points to the sensitive file.
        let symlink_path = "malicious_link.txt";
        let _ = fs::remove_file(symlink_path);
        symlink(sensitive_file, symlink_path).expect("Failed to create symlink");
        
        // Call the processing function with the symlink.
        // Expected result: In the fixed version, this call should fail by denying access.
        let result = process_file(symlink_path);
        assert!(result.is_err(), "Access via symbolic link should be restricted");

        // Clean up the files created for the test.
        let _ = fs::remove_file(sensitive_file);
        let _ = fs::remove_file(symlink_path);
    }
}