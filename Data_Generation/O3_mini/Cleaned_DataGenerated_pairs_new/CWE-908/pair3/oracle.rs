#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    #[test]
    fn test_configuration_validity() {
        let config = Configuration::create_instance();
        assert!(config.is_active(), "Configuration should be active");
        let handle = thread::spawn(move || {
            config.compute()
        });
        let result = handle.join().unwrap();
        assert_eq!(result, 84, "Computed result should be 84");
    }
}
