///////////////////////////////
// Test Oracle Function
///////////////////////////////

// This test oracle function is intended to be compiled separately (e.g., as part of a test suite).
// It invokes the processing operation with a controlled index value (32).
// In the vulnerable version, the background thread causes the invariant to fail,
// leading the function to panic. In the fixed version, the invariant is preserved,
// allowing the function to complete normally.

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        // The test asserts that the processing operation completes without panicking.
        // In the vulnerable version, this test will fail (catching a panic),
        // whereas in the fixed version, it will pass.
        let result = std::panic::catch_unwind(|| {
            let proc = Processor::new();
            proc.execute(32);
        });
        assert!(result.is_ok(), "Processing operation panicked due to invariant violation");
    }
}