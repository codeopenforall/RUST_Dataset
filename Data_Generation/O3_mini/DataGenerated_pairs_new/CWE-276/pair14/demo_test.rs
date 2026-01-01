#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::MetadataExt;
    
    // This test oracle executes the main function (from the code under test) to generate "output.txt"
    // and then verifies that the file permission bits are correctly set to 0o600.
    #[test]
    fn permission_test() {
        // Clean up any preexisting file.
        let _ = fs::remove_file("output.txt");
        
        // Run the main function which generates the file.
        crate::main();
        
        // Retrieve file metadata and check permissions.
        let meta = fs::metadata("output.txt").expect("output.txt should exist");
        let perms = meta.mode() & 0o777;
        
        // The secure version should have permissions 0o600.
        // If the vulnerable code runs, perms will be 0o777 and the test will fail.
        assert_eq!(perms, 0o600, "File permissions are insecure: expected 0o600, got {:o}", perms);
    }
}