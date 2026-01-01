/*
   Test Oracle:
   This test function checks that when the system creates a file the resulting permission mode is secure.
   It calls the public 'run' function (which writes to the file) and then verifies the file metadata.
   The test expects the file permission to be 0o600. For the vulnerable version (using 0o777),
   this test will fail, while it will pass for the fixed version.
*/

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::MetadataExt;
    use std::path::Path;

    // Depending on which version is compiled, the run function should be linked appropriately.
    // Ensure that the target file is removed before and after the test.
    #[test]
    fn test_file_permissions() {
        let test_path = "/tmp/insecure_test.txt";
        if Path::new(test_path).exists() {
            fs::remove_file(test_path).expect("Failed to remove previous test file");
        }

        // Call the library's run function to create and write the file.
        // This function is expected to create the file with secure permissions in the fixed version.
        crate::run(test_path);

        let metadata = fs::metadata(test_path).expect("Failed to retrieve file metadata");
        // Extract only the permission bits.
        let mode = metadata.mode() & 0o777;
        // The expected secure permission is 0o600.
        assert_eq!(
            mode, 0o600,
            "File was created with insecure permissions: expected 0o600, got {:o}",
            mode
        );

        // Clean up after verification.
        fs::remove_file(test_path).expect("Failed to remove test file");
    }
}