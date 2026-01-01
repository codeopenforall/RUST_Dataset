//////////////////////////////
// Test Oracle Functionality//
//////////////////////////////
#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::MetadataExt;
    // This test creates the file using the main function of the binary,
    // then verifies that the file permissions are correctly set to 0o640.
    #[test]
    fn test_file_permissions() {
        // Clean-up any previous test files.
        let _ = fs::remove_file("testfile.txt");
        // Call the main function to create the file.
        crate::main();
        // Retrieve file metadata.
        let meta = fs::metadata("testfile.txt").expect("File not found");
        let mode = meta.mode() & 0o777;
        // The expected secure mode is 0o640.
        assert_eq!(mode, 0o640, "Insecure file permissions detected");
    }
}