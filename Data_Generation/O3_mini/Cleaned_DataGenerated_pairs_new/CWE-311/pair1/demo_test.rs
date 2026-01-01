#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;

    // This test oracle checks that the sensitive secret is not stored in clear text.
    // For the vulnerable version, the file "secret.txt" will contain the plaintext
    // "my_super_secret", so the assertion will fail. In the fixed version, the file
    // contents are encrypted, so they will not match the plaintext.
    #[test]
    fn test_sensitive_storage_encryption() {
        // Remove any existing file.
        let _ = fs::remove_file("secret.txt");

        // Execute the program's main function.
        crate::main();

        // Read the contents of the file.
        let data = fs::read("secret.txt").expect("File 'secret.txt' should exist");
        let expected = b"my_super_secret";

        // The test asserts that the file content should NOT equal the sensitive plaintext.
        assert_ne!(data, expected, "Sensitive data is stored in clear text!");
    }
}