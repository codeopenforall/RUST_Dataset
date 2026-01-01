///////////////////////////////////////////////
// Test Oracle Function for Double Free Check
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;

    // This function wraps the execution of the main logic.
    // In the vulnerable version the execution will panic due to double free.
    // In the fixed version it will run without panic.
    pub fn run_main() {
        // Invoke the main function defined in the crate.
        // It is assumed that the crate provides a 'main' function.
        crate::main();
    }

    #[test]
    fn test_double_free() {
        let result = panic::catch_unwind(|| {
            run_main();
        });
        // When testing the vulnerable implementation, a panic is expected.
        // For the corrected implementation, no panic should occur.
        // Adjust the expected behavior by enabling one of the features.
        if cfg!(feature = "vulnerable") {
            assert!(result.is_err(), "Expected panic due to double free in vulnerable version");
        } else {
            assert!(result.is_ok(), "Did not expect panic in corrected version");
        }
    }
}