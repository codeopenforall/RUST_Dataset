#[cfg(test)]
mod tests {
    use std::fs;
    // To ensure the test is reproducible, remove any pre-existing output file.
    fn cleanup() {
        let _ = fs::remove_file("output.txt");
    }

    // The common oracle test function.
    // This test calls the main function from the binary under test and checks
    // whether the sensitive data is stored in encrypted form.
    // For the vulnerable version, the file will contain the plaintext secret,
    // causing the assertion to fail. The fixed version will pass as the data is encrypted.
    #[test]
    fn test_sensitive_storage() {
        cleanup();
        // Call the main function of the binary.
        // In integration scenarios, this could be executed in a subprocess.
        // Here we assume a direct call to main() is possible.
        crate::main();
        let content = fs::read("output.txt").expect("Failed to read output file");

        // The expected sensitive string in plaintext.
        let sensitive = b"SensitiveSecret";

        // If data is unencrypted, it will match the plaintext.
        // To pass, the stored content must not equal the raw sensitive data.
        assert_ne!(
            content, sensitive,
            "Test failed: Sensitive data is stored in plaintext"
        );
    }
}