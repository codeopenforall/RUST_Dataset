//////////////////////////
// Test Oracle Function
//////////////////////////

// This test oracle is independent from the main application code.
// It verifies that running the application does not trigger the invariant's assertion.
// In the vulnerable configuration, the reachable assertion fires and causes a panic,
// so this test will fail; in the fixed configuration, the test passes.

#[cfg(test)]
mod tests {
    use std::panic;

    #[test]
    fn test_invariant_maintained() {
        // Call the exported start function and catch unwinding panics.
        // In the fixed version, start() returns Ok(()), whereas the vulnerable version panics.
        let result = panic::catch_unwind(|| {
            // Depending on which configuration is being tested,
            // this function call should either panic or run successfully.
            crate::start();
        });
        assert!(
            result.is_ok(),
            "Execution panicked: invariant check triggered a reachable assertion failure"
        );
    }
}