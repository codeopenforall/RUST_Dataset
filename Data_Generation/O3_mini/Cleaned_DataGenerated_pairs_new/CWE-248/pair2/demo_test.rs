#[cfg(test)]
mod tests {
    use std::panic;

    // The test oracle calls the main entry point.
    // In the vulnerable version, a panic will occur causing the test to fail.
    // In the fixed version, the panic is caught, and the application exits normally.
    #[test]
    fn test_entry() {
        // Capture any panic from main.
        let result = panic::catch_unwind(|| {
            // Call the entry point; this should not panic in the fixed version.
            crate::main();
        });
        // The test asserts that main did not panic.
        assert!(result.is_ok(), "Expected normal termination, but a panic occurred.");
    }
}