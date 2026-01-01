//////////////////////////////////////////
// Test Oracle for detecting uninitialized resource misuse
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    // Test function for configuration instance.
    // The test creates an instance using the 'create_instance' method
    // and asserts that the configuration is fully valid.
    // For the vulnerable version, uninitialized 'active' will cause
    // the assertion to (likely) fail or exhibit undefined behavior.
    #[test]
    fn test_configuration_validity() {
        let config = Configuration::create_instance();
        // The test expects the 'active' flag to be true.
        // In the vulnerable code, this may be uninitialized.
        assert!(config.is_active(), "Configuration should be active");

        // Additionally, concurrent usage should yield a computed result.
        let handle = thread::spawn(move || {
            config.compute()
        });
        let result = handle.join().unwrap();
        // Expected computed result: 42 * 2 = 84 when active is true.
        assert_eq!(result, 84, "Computed result should be 84");
    }
}