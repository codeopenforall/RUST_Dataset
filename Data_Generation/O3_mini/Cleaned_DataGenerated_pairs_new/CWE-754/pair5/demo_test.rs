//////////////////////////////////////////////
// Test oracle for verifying the behavior against
// improper exceptional condition handling.
// This test will fail for the vulnerable version (which returns Ok(0))
// and pass for the corrected version (which returns an error on timeout).
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Processor;

    #[test]
    fn test_timeout_error() {
        let instance = Processor::new();
        let result = instance.execute();
        match result {
            Err(message) => {
                // In the fixed version, we expect the specific timeout error message.
                assert_eq!(message, "Operation timed out", "Expected a timeout error.");
            },
            Ok(val) => {
                panic!("Test failed: Expected a timeout error, but received value: {}", val);
            },
        }
    }
}