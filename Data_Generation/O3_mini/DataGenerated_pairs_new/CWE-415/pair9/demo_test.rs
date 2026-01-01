////////////////////////////////////////////////////////////////
// Test oracle function to differentiate the behavior.
// When running the vulnerable version the double release will trigger a panic,
// whereas the corrected version will execute without panicking.
#[cfg(test)]
mod oracle {
    use std::panic;

    // The run_app() function is invoked in a catch_unwind block.
    // For the vulnerable version, a panic ("Double free detected") will occur,
    // causing this test to fail. For the corrected version, no panic occurs, so the test passes.
    #[test]
    fn test_item_behavior() {
        let result = panic::catch_unwind(|| {
            // Assumes run_app() is the entry function for the application.
            crate::run_app();
        });
        assert!(result.is_ok(), "Test oracle: double release error detected.");
    }
}