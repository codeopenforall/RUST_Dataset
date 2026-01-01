///////////////////////////////////////////////////////////////
// Test Oracle: Permission Verification Test
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    // This test calls the main setup function and then verifies that the created file's permission
    // is exactly 0o600. The test will fail if the file has insecure (overly permissive) permissions.
    #[test]
    fn file_permission_test() {
        // Remove any existing file.
        let _ = fs::remove_file("config.txt");

        // Call the main function of the binary.
        // This assumes that the binary's main creates the file "config.txt".
        crate::main();

        let metadata = fs::metadata("config.txt").expect("File not created");
        // Mask out other bits and obtain permission bits.
        let mode = metadata.permissions().mode() & 0o777;
        assert_eq!(
            mode, 0o600,
            "File permissions are insecure. Expected 0o600 but found {:o}",
            mode
        );
    }
}