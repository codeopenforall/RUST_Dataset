#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_extraction_valid_and_invalid() {
        let proc_inst = Processor;

        // Valid input: extract "Example" from the beginning.
        match proc_inst.process(0, 7) {
            Ok(result) => assert_eq!(result, "Example", "Valid extraction result mismatch"),
            Err(e) => panic!("Unexpected error on valid input: {}", e),
        }

        // Invalid input: intentionally request an out-of-bound slice.
        // For the fixed version, this must return an Err with a specific message.
        // The vulnerable version may panic unexpectedly or produce undefined data.
        let invalid_output = panic::catch_unwind(|| proc_inst.process(65, 10));
        match invalid_output {
            Ok(res) => {
                match res {
                    Ok(s) => panic!("Test failed: extraction with invalid input succeeded with '{}'", s),
                    Err(msg) => assert_eq!(msg, "Invalid input: offset and length exceed data bounds", "Unexpected error message"),
                }
            },
            Err(_) => panic!("Test failed: extraction with invalid input panicked in the vulnerable version"),
        }
    }
}