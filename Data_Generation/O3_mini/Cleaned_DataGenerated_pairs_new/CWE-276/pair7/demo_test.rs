///////////////////////
// Test Oracle Code
///////////////////////
#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    // NOTE:
    // The Test Oracle expects that the file created by the persist function
    // has secure permissions: 0o600. For the insecure variant, this test will fail,
    // while for the secure variant, it should pass.
    //
    // This test is independent and should be compiled separately from the main codes.
    //
    // Both versions are expected to provide a DataStore struct with a persist method.

    // A dummy trait to generalize the interface for testing.
    pub trait Storage {
        fn persist(&self, name: &str, content: &[u8]) -> std::io::Result<()>;
    }

    // Assume DataStore implements Storage in both versions.
    use super::DataStore;

    #[test]
    fn file_permissions_check() {
        let test_filename = "test_permission.txt";
        let base_dir = "/tmp";

        // Remove any preexisting file.
        let test_path = format!("{}/{}", base_dir, test_filename);
        let _ = fs::remove_file(&test_path);

        let ds = DataStore::new(base_dir);
        ds.persist(test_filename, b"oracle test").expect("Failed to write test file");

        let metadata = fs::metadata(&test_path).expect("Failed to fetch metadata");
        let perm_mode = metadata.permissions().mode() & 0o777;

        // The expected secure permission is 0o600.
        assert_eq!(perm_mode, 0o600, "File permissions are not secure");

        // Cleanup.
        let _ = fs::remove_file(&test_path);
    }
}