#[cfg(test)]
mod tests {
    use super::transmit_secret;

    #[test]
    fn test_encrypted_transmission() {
        let transmitted = transmit_secret();
        // In the vulnerable version, the secret is sent in clear text ("supersecret").
        // In the fixed version, it is encrypted and encoded as hex.
        // This test fails if the transmitted data equals the clear text.
        assert_ne!(transmitted, "supersecret", "Sensitive data transmitted in clear text!");
    }
}