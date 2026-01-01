/*
Test oracle: This unit test is designed to verify proper error handling.

The test creates a new instance of the Handler and provides an input slice that exceeds
the allocated buffer capacity. The expected behavior is that process() returns an error.

For the vulnerable version, the test will fail because process() incorrectly returns Ok(()).
For the corrected version, the test will pass as it correctly propagates the error.
*/

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn integration_test_oracle() {
        // Create an instance with a 64-byte buffer
        let mut instance = Handler::new();
        // An input that exceeds the buffer capacity (100 bytes vs 64)
        let data = vec![1u8; 100];
        let result = instance.process(&data);
        assert!(result.is_err(), "Expected error due to input exceeding buffer capacity");
    }
}