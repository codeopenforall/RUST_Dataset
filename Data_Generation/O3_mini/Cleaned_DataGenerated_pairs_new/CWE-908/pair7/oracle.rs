#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    #[test]
    fn config_validation_test() {
        let cfg = initialize_config();
        let shared_cfg = Arc::new(cfg);
        let handle = thread::spawn({
            let shared_clone = Arc::clone(&shared_cfg);
            move || {
                assert!(
                    validate_config(&shared_clone),
                    "Configuration validation failed in secondary thread."
                );
            }
        });
        handle.join().unwrap();
        assert!(
            validate_config(&shared_cfg),
            "Configuration validation failed in main thread."
        );
    }
}
