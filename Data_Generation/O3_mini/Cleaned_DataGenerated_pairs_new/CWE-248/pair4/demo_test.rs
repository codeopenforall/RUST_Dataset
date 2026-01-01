//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::process;

    // This test is designed to fail for the vulnerable version (which propagates the panic)
    // and pass for the corrected version (which catches and handles the panic gracefully).
    #[test]
    fn concurrency_recovery_oracle() {
        // Trigger the unsafe operation with a non-zero value.
        // In a secure implementation, this should be captured and processed as an Ok result.
        let result = process(1);
        assert!(result.is_ok(), "Expected Ok result, but got an error: {:?}", result);
    }
}