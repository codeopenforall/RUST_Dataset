#[cfg(test)]
mod oracle {
    use std::panic;
    #[test]
    fn test_item_behavior() {
        let result = panic::catch_unwind(|| {
            crate::run_app();
        });
        assert!(result.is_ok(), "Test oracle: double release error detected.");
    }
}
