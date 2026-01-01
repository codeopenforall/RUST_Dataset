#[cfg(test)]
mod tests {
    use super::process_output;
    
    #[test]
    fn test_transmission_security() {
        // The original plaintext secret.
        let plaintext = "SuperSecretPassword";
        let output = process_output();
        
        // For a secure implementation, the output must differ from the plaintext.
        // In an insecure (vulnerable) scenario the test will fail.
        assert_ne!(output, plaintext, "Sensitive data was transmitted in cleartext.");

        // For demonstration purposes, apply the same XOR encryption on the plaintext.
        let key = b'K';
        let expected: String = plaintext.bytes().map(|b| (b ^ key) as char).collect();
        assert_eq!(output, expected, "The encryption output does not match the expected result.");
    }
}