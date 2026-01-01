//////////////////////////////////////////////////////////////
// Test Oracle Function for Verifying the Signature Check Behavior
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // The test operates by attempting verification with an invalid domain.
    // In a secure implementation, the result must be false.
    // In the vulnerable implementation, the flawed logic incorrectly returns true.
    #[test]
    fn test_verification_oracle() {
        let verifier = Arc::new(Verifier);
        let message = "Important message";
        let signature = b"valid_signatur"; // 14 bytes correct dummy signature.
        let wrong_domain = "invalid_domain"; // This should lead to failure in a fixed version.

        let result = verifier.validate(message, signature, wrong_domain);
        // The test asserts that the signature must be rejected when the domain is incorrect.
        assert!(!result, "Verification should fail for an incorrect domain");
    }
}