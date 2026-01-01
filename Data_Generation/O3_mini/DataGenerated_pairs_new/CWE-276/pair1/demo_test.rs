////////////////////////////////////////////////////////////////////////////////////////////////////
// Test Oracle Function
// This test creates the file by invoking the main functionality and then inspects the file's permissions.
// It asserts that the file permission mode equals 0o600 (owner read/write only). In the vulnerable
// version, the file is created with 0o777, causing the assertion to fail.
// The oracle function is independent of the main implementations.
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn test_permissions() {
        // Execute the main function of the target binary.
        // Assuming the binary creates a file named "server_output.txt".
        // Clean up beforehand.
        let _ = fs::remove_file("server_output.txt");
        // Call main (this works if the main is accessible; otherwise, simulate file creation).
        crate::main();

        let meta = fs::metadata("server_output.txt")
                        .expect("Failed to get metadata of server_output.txt");
        let mode = meta.permissions().mode() & 0o777;
        // The secure version should have mode 0o600.
        assert_eq!(mode, 0o600, "File was created with insecure permissions: {:o}", mode);
    }
}