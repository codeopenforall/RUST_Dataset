#[cfg(test)]
mod tests {
    use super::*;
    const XOR_KEY: u8 = 0xAA;
    fn xor_cipher(data: &[u8]) -> Vec<u8> {
        data.iter().map(|b| b ^ XOR_KEY).collect()
    }

    // The test oracle creates a configuration instance and checks the output of the transmit() function.
    // For the vulnerable version, transmit() returns the plaintext, so the test will panic.
    // For the fixed version, transmit() returns the encrypted bytes (via XOR), which the test asserts.
    #[test]
    fn test_transmission() {
        let config = Config::new("SecretPassword123");
        let transmitted = config.transmit();

        // The cleartext bytes for the sensitive data.
        let cleartext = b"SecretPassword123".to_vec();

        // Test condition:
        // If the transmitted data equals the cleartext, then the secret is unencrypted and the test must fail.
        // Otherwise, for the fixed implementation, the transmitted data should equal the XOR encrypted result.
        if transmitted == cleartext {
            panic!("Test failed: Sensitive data was transmitted without encryption.");
        } else {
            let expected = xor_cipher(&cleartext);
            assert_eq!(transmitted, expected, "Test passed: Sensitive data is encrypted properly.");
        }
    }
}