///////////////////////////////////////////////////////////////
// Test Oracle Function:
// This unit test is meant to be compiled and run separately
// (e.g., via cargo test). When run against the vulnerable
// version, it will fail because the panic escapes; for the fixed
// version, the panic is caught and the test passes.
///////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::panic;

    // The test calls the main function of the module.
    // For the vulnerable variant, main() panics and the test
    // asserts a failure; for the fixed version, main() executes
    // gracefully.
    #[test]
    fn oracle() {
        let result = panic::catch_unwind(|| {
            // Call the program entry point.
            super::main();
        });
        // Expecting no panic (i.e. Ok) for the fixed code.
        assert!(result.is_ok(), "Program terminated abnormally");
    }
}