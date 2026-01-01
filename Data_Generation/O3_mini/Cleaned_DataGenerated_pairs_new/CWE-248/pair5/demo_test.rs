#[cfg(test)]
mod tests {
    use std::panic;
    // Import the application entry point from the corresponding module.
    // For these tests, adjust the module path as needed.
    use crate::run_app;

    #[test]
    fn test_oracle() {
        // The test calls run_app() inside a catch_unwind.
        // In the problematic implementation, a panic is expected to propagate,
        // making the test fail. In the corrected version, the panic is caught and
        // run_app() completes without unwinding further.
        let result = panic::catch_unwind(|| {
            run_app();
        });
        assert!(result.is_ok(), "The application did not handle the panic gracefully");
    }
}