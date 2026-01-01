#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_signature_verification() {
        let packet = DataPacket::new("Hello", "invalid_signature", "reverse");
        assert!(!packet.validate(), "Test Oracle: Signature was improperly accepted.");
    }
}
