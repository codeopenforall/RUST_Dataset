#[cfg(test)]
mod tests {
    use std::fs::{metadata, remove_file};
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    // This test oracle is designed to work with two different builds:
    // - In the insecure build, the file "tempfile.txt" is created with mode 0o777.
    // - In the secure build, the file "tempfile.txt" is created with mode 0o600.
    //
    // The test will fail if the file permissions are insecure (0o777) and pass if they are secure (0o600).

    #[test]
    fn test_file_permissions() {
        // Remove any existing file to ensure a clean test.
        let path = "tempfile.txt";
        if Path::new(path).exists() {
            remove_file(path).unwrap();
        }

        // Invoke the main function of the binary.
        // Note: The main function in the compiled binary (either insecure or secure version) is assumed to create the file.
        crate::main().expect("Execution of main failed");

        // Retrieve file metadata and check its permissions.
        let meta = metadata(path).expect("Unable to read metadata for tempfile.txt");
        let perms = meta.permissions().mode() & 0o777;
        // Assert that the file permissions match the secure value.
        assert_eq!(perms, 0o600, "File permissions are insecure: expected 0o600, got {:o}", perms);
    }
}