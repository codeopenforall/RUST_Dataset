#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_signature_verification() {
        let message_text = "Test".to_string();
        let signature: Vec<u8> = message_text.bytes().rev().collect();
        let payload = Message::new(message_text, "fake".to_string(), signature);
        let result = process(payload);
        assert!(!result, "Payload with invalid algorithm should be rejected");
    }
}
