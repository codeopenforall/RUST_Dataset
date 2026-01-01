/*
   Test oracle: This unit test is designed to detect the vulnerability.
   It calls process_input with a negative value which, in the vulnerable version,
   triggers a panic due to improper error handling, whereas in the fixed version, it 
   gracefully returns an error code (-1).

   The test structure below should be added as a separate test module and built/executed 
   independently from the vulnerable and fixed binaries.
*/
#[cfg(test)]
mod tests {
    use std::panic;

    // Import the process_input function from the binary under test.
    // For the vulnerable build, process_input returns i32 and panics on error.
    // For the fixed build, process_input returns -1 gracefully on error.
    use super::process_input;

    #[test]
    fn test_oracle() {
        // Using negative input to trigger error handling.
        let test_input = -5;

        // Catch unwinding to determine if the function panics (vulnerable) or returns gracefully (fixed).
        let result = panic::catch_unwind(|| process_input(test_input));

        if result.is_err() {
            // The vulnerable version will panic, causing the test to fail.
            panic!("Test oracle: Vulnerability detected - function panicked on invalid input.");
        } else {
            let output = result.unwrap();
            // The fixed version is expected to return -1 for a negative input.
            assert_eq!(output, -1, "Test oracle: Unexpected output. Expected -1 for invalid input.");
        }
    }
}