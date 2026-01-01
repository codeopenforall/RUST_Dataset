//////////////////////////////
// Test Oracle Function     //
// This test should fail   //
// when run against the     //
// vulnerable version and   //
// pass for the corrected   //
// version.                 //
//////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    // Import run_system from the target module.
    // For the vulnerable version, run_system() will panic; for the fixed version, it will complete gracefully.
    use super::run_system;

    #[test]
    fn test_run_system_stability() {
        // If run_system() panics unexpectedly, catch_unwind returns Err.
        let result = panic::catch_unwind(|| {
            run_system();
        });
        // Expectation: No panic should occur.
        assert!(result.is_ok(), "run_system() panicked unexpectedly");
    }
}