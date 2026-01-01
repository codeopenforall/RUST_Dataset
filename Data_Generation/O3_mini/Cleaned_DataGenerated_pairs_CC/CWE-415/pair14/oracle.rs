#[cfg(test)]
mod tests {
    use std::panic;
    use crate::app_run;
    #[test]
    fn test_memory_release() {
        let result = panic::catch_unwind(|| {
            app_run();
        });
        assert!(result.is_ok(), "app_run panicked due to double free vulnerability.");
    }
}
