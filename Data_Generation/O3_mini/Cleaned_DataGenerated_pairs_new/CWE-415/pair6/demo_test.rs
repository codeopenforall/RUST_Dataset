////////////////////////////////////////////////////////////
// Test Oracle: This function serves as a unit test.
// It calls the main function from the compiled binary and uses
// std::panic::catch_unwind to detect if a double free panic occurs.
//
// When compiled with the problematic (unsafe) code, the test should
// capture a panic (indicating the double free was triggered).
// When compiled with the corrected code, no panic should occur.
//
// Use a feature flag "issue" to differentiate between the two builds.
// Do not include this oracle code in the main source files.
////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    
    #[test]
    fn test_resource_double_free() {
        // Invoke the binary's main function within a catch_unwind.
        // The expected outcome depends on a feature flag.
        let result = panic::catch_unwind(|| {
            crate::main();
        });
        // If the code is compiled with the issue (unsafe deallocation)
        // enabled, a panic is expected (test passes if panic is caught).
        // Otherwise the corrected code should complete without panicking.
        #[cfg(feature = "issue")]
        assert!(result.is_err(), "Expected panic due to double free");
        #[cfg(not(feature = "issue"))]
        assert!(result.is_ok(), "Expected successful run without panic");
    }
}