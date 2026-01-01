#[cfg(test)]
mod tests {
    use std::fs;
    #[test]
    fn test_oracle() {
        // Set up the configuration file with malformed (non-numeric) data.
        fs::write("setting.conf", "invalid_data").expect("Failed to write test configuration");
        // The expected behavior is that the configuration function returns an error.
        let result = crate::configure();
        assert!(result.is_err(), "Expected error due to invalid configuration, but got Ok");
    }
}