#[cfg(test)]
mod tests {
    use std::panic;

    // This test uses an input that would normally trigger the assertion in the insecure version.
    // When run against the insecure implementation, the test will panic (and thus fail),
    // while with the secure implementation the check prevents the panic.
    #[test]
    fn test_run_app_threshold() {
        // Input designed to cause the counter to reach or exceed the threshold.
        let input = "100".to_string();

        // Capturing the run_app call inside a catch_unwind block.
        let result = panic::catch_unwind(|| {
            // Assumes a function `run_app` exists in the current crate.
            crate::run_app(input)
        });

        // For the secure version, result is Ok and the counter remains below threshold.
        // For the insecure version, the assertion fires and result is an error.
        assert!(result.is_ok(), "Application panicked unexpectedly");
        let counter = result.unwrap();
        assert!(counter < 100, "Counter exceeded safe threshold");
    }
}