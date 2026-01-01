///////////////////////////////////////////////
// Test Oracle: This unit test is applied externally.
// It calls the public critical_run() function and asserts that it completes normally.
// For the vulnerable implementation, this test will catch a panic and fail.
// For the corrected implementation, critical_run() will execute without panicking.
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test executes the critical function and expects no panic.
    #[test]
    fn test_critical_run() {
        let result = panic::catch_unwind(|| {
            critical_run();
        });
        // The invariant violation should not cause a panic in the corrected code.
        assert!(result.is_ok(), "critical_run() panicked unexpectedly");
    }
}