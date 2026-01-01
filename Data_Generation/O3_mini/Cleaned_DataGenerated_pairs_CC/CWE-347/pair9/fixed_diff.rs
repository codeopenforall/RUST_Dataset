    fn derive(&self, message: &[u8]) -> Vec<u8> {
        let total: u32 = message.iter().chain(self.pubkey.iter())
                                .map(|&b| b as u32)
                                .sum();
        vec![(total % 256) as u8]
    }
    fn check(&self, message: &[u8], signature: &[u8]) -> bool {
        let expected = self.derive(message);
        expected == signature
    let proper_signature = {
        let guard = checker.lock().unwrap();
        guard.derive(&message)
    };
        let sig = proper_signature.clone();
        assert!(result, "Authentication should pass with a valid signature.");
