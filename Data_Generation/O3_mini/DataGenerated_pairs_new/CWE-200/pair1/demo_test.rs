///////////////////////////////////////////////////////////////
// Test Oracle Function: Validate that sensitive info is not leaked.
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        // Create an instance with a known secret.
        let sample = Data::new("SuperSecret123", 99);
        // Call process_data with trigger=true to simulate an error.
        let err_msg = process_data(&sample, true).unwrap_err();
        // Assert that the error message does not include the sensitive secret.
        // In the vulnerable version, the secret "SuperSecret123" would be present.
        // In the corrected version, the error message is generic.
        assert!(
            !err_msg.contains("SuperSecret123"),
            "Sensitive information should not be exposed in the error message!"
        );
    }
}