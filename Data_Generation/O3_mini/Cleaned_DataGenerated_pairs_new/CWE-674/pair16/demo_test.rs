////////////////////////////////////////////
// Test Oracle for Recursion Safety
// This test function builds a deeply nested input string (more than MAX_DEPTH)
// and invokes the 'process' method from the Processor.
// For the vulnerable version, the test is expected to panic (or unwind) due to a stack overflow.
// For the fixed version, the function should return an error with the message "Recursion limit reached".
// This test uses std::panic::catch_unwind to capture panics.
////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Processor;
    use std::panic;

    #[test]
    fn recursion_oracle() {
        // Create a deeply nested input string (exceeds the safe recursion limit).
        let depth = 1100;
        let input = "(".repeat(depth) + &")".repeat(depth);
        let proc = Processor;

        let result = panic::catch_unwind(|| proc.process(&input));

        match result {
            // For the fixed version, we expect a graceful error.
            Ok(Ok(err)) => {
                // If the process function erroneously returns Ok(_) when it should error, fail the test.
                panic!("Expected an error, but got a depth value: {}", err);
            },
            Ok(Err(e)) => {
                // Fixed version: should produce an explicit error.
                assert_eq!(e, "Recursion limit reached", "Expected recursion limit error");
            },
            // For the vulnerable version, the function is expected to panic.
            Err(_) => {
                panic!("Function panicked due to uncontrolled recursion");
            }
        }
    }
}