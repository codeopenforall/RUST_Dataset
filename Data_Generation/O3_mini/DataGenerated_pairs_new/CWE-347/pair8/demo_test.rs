#[cfg(test)]
mod tests {
    // Import all definitions from the current module.
    use super::*;

    #[test]
    fn test_signature_verification() {
        // Prepare a payload with an invalid algorithm.
        let message_text = "Test".to_string();
        // Compute the signature that would be valid if "ed25519" were used.
        let signature: Vec<u8> = message_text.bytes().rev().collect();
        // Intentionally set the algorithm to an incorrect value.
        let payload = Message::new(message_text, "fake".to_string(), signature);

        let result = process(payload);
        // The correct behavior is to reject a payload with an unexpected algorithm.
        // Therefore, the result should be false.
        assert!(!result, "Payload with invalid algorithm should be rejected");
    }
}