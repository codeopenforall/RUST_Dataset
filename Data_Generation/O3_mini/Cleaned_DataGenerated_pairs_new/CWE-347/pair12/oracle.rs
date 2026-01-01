#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    #[test]
    fn test_verification_oracle() {
        let verifier = Arc::new(Verifier);
        let message = "Important message";
        let signature = b"valid_signatur"; 
        let wrong_domain = "invalid_domain"; 
        let result = verifier.validate(message, signature, wrong_domain);
        assert!(!result, "Verification should fail for an incorrect domain");
    }
}
