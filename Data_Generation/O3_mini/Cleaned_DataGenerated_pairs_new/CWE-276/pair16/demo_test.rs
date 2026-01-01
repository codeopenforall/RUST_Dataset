#[cfg(test)]
mod tests {
    use std::fs::metadata;
    #[test]
    fn test_file_permissions() {
        // Remove the existing file to ensure a clean state before test execution.
        let _ = std::fs::remove_file("/tmp/test_insecure_file.txt");
        // Execute the main function which writes the file.
        crate::main();
        let meta = metadata("/tmp/test_insecure_file.txt").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            // Extract only permission bits.
            let mode = meta.mode() & 0o777;
            // For the secure implementation, the mode should be 0o600.
            assert_eq!(mode, 0o600, "Insecure file permissions detected");
        }
    }
}