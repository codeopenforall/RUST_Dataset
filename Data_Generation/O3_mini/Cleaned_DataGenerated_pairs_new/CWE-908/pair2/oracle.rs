#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;
    use super::load_configuration;
    use super::Config;
    #[test]
    fn configuration_test() {
        let config = load_configuration();
        assert_eq!(config.threshold, 100, "Threshold field is incorrect.");
        assert_eq!(config.message, "default", "Message field is incorrect.");
        assert!(config.validate(), "Configuration did not validate as expected.");
    }
    #[test]
    fn concurrent_access_test() {
        let config_shared = Arc::new(load_configuration());
        let config_for_thread = Arc::clone(&config_shared);
        let handle = thread::spawn(move || {
            assert!(config_for_thread.validate());
        });
        handle.join().unwrap();
    }
}
