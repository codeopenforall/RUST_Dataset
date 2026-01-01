#[cfg(test)]
mod tests {
    use std::panic;
    #[test]
    fn test_run_app_integrity() {
        let result = panic::catch_unwind(|| {
            crate::run_app();
        });
        assert!(result.is_ok(), "Application panicked due to double free vulnerability");
    }
}
