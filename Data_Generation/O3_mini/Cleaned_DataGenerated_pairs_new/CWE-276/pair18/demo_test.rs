#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    // Helper function to execute the application logic.
    // It is assumed that the tested binary's main creates "data.txt".
    fn execute_app() {
        // Call the main function from the application.
        // This will create/update "data.txt" with specific permissions.
        crate::main();
    }

    #[test]
    fn test_file_permissions() {
        let file_path = "data.txt";
        // Ensure a clean state for the test
        let _ = fs::remove_file(file_path);
        execute_app();
        let metadata = fs::metadata(file_path).expect("Unable to read file metadata");
        let mode = metadata.permissions().mode() & 0o777;
        // The contract requires that the file permissions must be 0o600.
        // For the insecure implementation the test will fail.
        assert_eq!(mode, 0o600, "File permissions are insecure! Found: {:o}", mode);
    }
}