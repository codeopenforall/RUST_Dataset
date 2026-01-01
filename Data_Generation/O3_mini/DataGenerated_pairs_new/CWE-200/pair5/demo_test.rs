/////////////////// Test Oracle ///////////////////
// This test oracle function asserts that a call to process() with an invalid value
// does not expose the secret ("SuperSecret123") in its error message. For the vulnerable
// version, this test will fail due to the sensitive information leak, while for the fixed
// version, it will pass.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensitive_data_not_exposed() {
        let instance = SecureData::new("SuperSecret123".to_string());
        let response = instance.process(101);
        match response {
            Err(err_msg) => {
                // The oracle checks that the error message does NOT contain the sensitive string.
                assert!(
                    !err_msg.contains("SuperSecret123"),
                    "Test failed: sensitive data exposure detected"
                );
            },
            Ok(_) => panic!("Test failed: invalid input should produce an error"),
        }
    }
}