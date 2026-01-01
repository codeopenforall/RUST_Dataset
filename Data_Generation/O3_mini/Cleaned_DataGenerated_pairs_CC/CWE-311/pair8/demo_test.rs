#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    // This oracle test is designed to detect if the stored configuration is left in cleartext.
    // It creates a configuration, triggers the storage method, reads the stored file,
    // and asserts that the stored data does not match the plain concatenation of secrets.
    #[test]
    fn test_sensitive_storage() {
        // Clean up any pre-existing file.
        let _ = fs::remove_file("config.txt");
        let cfg = Config::new(
            "SuperSecretAPIKey".to_string(),
            "VerySecretDBPassword".to_string()
        );
        cfg.store().expect("Store operation failed");
        let stored = fs::read("config.txt").expect("Failed to read stored configuration");

        let plain = "SuperSecretAPIKeyVerySecretDBPassword".as_bytes().to_vec();
        
        // For the insecure version, the stored data matches the plain text,
        // causing this assertion to fail. The secure version should produce encrypted output.
        assert_ne!(stored, plain, "Configuration stored in cleartext!");
    }
}