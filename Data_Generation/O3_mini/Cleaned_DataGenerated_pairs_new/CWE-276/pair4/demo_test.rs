#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    // This test calls the main function from the implementation.
    // For the insecure implementation, the permission bits are 0o777.
    // For the secure (fixed) version, they are 0o600.
    // The test expects secure file permissions. Running the insecure version should fail.
    #[test]
    fn test_file_permissions() {
        let file_path = "example.txt";
        // Ensure a clean slate.
        let _ = fs::remove_file(file_path);

        // Execute the main function from the implementation.
        crate::main();

        // Retrieve the file metadata.
        let metadata = fs::metadata(file_path).expect("Unable to fetch metadata.");
        let permissions = metadata.permissions().mode() & 0o777;

        // The secure version should enforce 0o600 permissions.
        assert_eq!(permissions, 0o600, "File permissions are insecure.");
    }
}