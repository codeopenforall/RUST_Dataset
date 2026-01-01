///////////////////////////////////////////////
// Test Oracle
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;

    // Bring the configuration loader into scope from the main module.
    use super::load_configuration;
    use super::Config;

    #[test]
    fn configuration_test() {
        // In both builds, load the configuration.
        let config = load_configuration();
        // Check that the numeric field is properly set.
        assert_eq!(config.threshold, 100, "Threshold field is incorrect.");
        // Check that the message field is properly set.
        assert_eq!(config.message, "default", "Message field is incorrect.");
        // Validate configuration; the vulnerable version leads to undefined behavior.
        assert!(config.validate(), "Configuration did not validate as expected.");
    }

    #[test]
    fn concurrent_access_test() {
        let config_shared = Arc::new(load_configuration());
        let config_for_thread = Arc::clone(&config_shared);
        let handle = thread::spawn(move || {
            // Validate configuration in a separate thread.
            assert!(config_for_thread.validate());
        });
        handle.join().unwrap();
    }
}