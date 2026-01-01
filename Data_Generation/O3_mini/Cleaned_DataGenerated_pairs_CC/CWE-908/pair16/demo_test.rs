////////////////////////////////////////////////////////////
// Test Oracle Function
////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // The test expects that compute() runs successfully and returns true in the fixed version.
    // In the vulnerable version, compute() will invoke undefined behavior (likely panicking).
    use super::compute;

    #[test]
    fn oracle() {
        let result = std::panic::catch_unwind(|| {
            compute()
        });
        // For the vulnerable version this should be an error (i.e. is_err() is true),
        // while for the fixed version the execution should succeed and return true.
        if result.is_err() {
            panic!("Test failed: The configuration processing panicked (vulnerability present)");
        }
        let valid = result.unwrap();
        assert!(valid, "Test failed: The configuration was not valid (unexpected value)");
    }
}