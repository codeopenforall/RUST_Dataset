/*
This test oracle function is designed to confirm the exploitability of the vulnerability in the vulnerable version 
and its remediation in the fixed version. The test calls the process() function with a deliberately invalid input (1500)
that should trigger a panic in the vulnerable variant, while the fixed variant should complete without panicking.

When linked with the vulnerable code, the test should fail because a panic occurs. Conversely, when linked with the fixed 
version, the test should pass.
*/
#[cfg(test)]
mod tests {
    use std::panic;

    // Import the process function.
    use super::process;

    #[test]
    fn oracle_test() {
        let result = panic::catch_unwind(|| {
            process(1500);
        });
        // In a secure implementation, the processing with invalid input should not panic.
        assert!(result.is_ok(), "Test oracle failed: operation panicked due to attacker-controlled input.");
    }
}