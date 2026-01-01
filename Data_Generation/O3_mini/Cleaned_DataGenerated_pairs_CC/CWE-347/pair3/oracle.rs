#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signature_verification_oracle() {
        let message = b"Test oracle message";
        let malicious_signature = vec![0x00; 64];
        assert!(!process_request(message, &malicious_signature),
                "The signature verification failed to reject a malicious signature.");
    }
}
