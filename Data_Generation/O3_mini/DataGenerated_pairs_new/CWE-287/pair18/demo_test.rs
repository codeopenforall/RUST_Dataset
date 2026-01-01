#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_authentication() {
        // In both implementations, the secret is "Password".
        // However, in the vulnerable version, the weak check only compares the first character,
        // so an input like "P12345" would incorrectly succeed.
        // For the fixed version, the input "P12345" must be rejected.
        //
        // Adjust the instantiation below to call the appropriate version:
        // For vulnerable version, use the unsafe check (vulnerable code),
        // and for the fixed version, use the safe check (fixed code).
        
        let auth = Authenticator::new("Password".to_string());
        
        // Test with a credential that only matches the first character.
        // The safe (fixed) implementation must reject this.
        assert_eq!(auth.process("P12345"), false, "Authentication should fail for weak token input");
        
        // Test with the correct credential.
        assert_eq!(auth.process("Password"), true, "Authentication should succeed for correct token input");
    }
}