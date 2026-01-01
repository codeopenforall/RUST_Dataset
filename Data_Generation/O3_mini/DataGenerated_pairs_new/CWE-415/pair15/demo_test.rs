///////////////////////////
// Test Oracle Code
///////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;

    // The simulate() function encapsulates the core logic.
    // For the vulnerable version, simulate() will panic due to double free.
    // For the fixed version, simulate() will complete without panicking.
    // Note: This test oracle is the same for both versions.
    #[test]
    fn test_simulate() {
        // Catch unwind to see if simulate() panics.
        let result = panic::catch_unwind(|| {
            // Call the simulation function from the main module.
            crate::simulate();
        });
        // For the vulnerable version, the test should fail (result.is_err()).
        // For the fixed version, the test should pass (result.is_ok()).
        assert!(result.is_ok(), "simulate() panicked, indicating a double free vulnerability");
    }
}