const DOMAIN: &[u8] = b"DOC"; 
    fn compute_expectation(&self, key: &[u8; 32]) -> [u8; 32] {
            let domain_byte = DOMAIN[i % DOMAIN.len()];
            let content_byte = bytes[i % bytes.len()];
            outcome[i] = content_byte ^ key[i] ^ domain_byte;
        let expected = self.compute_expectation(key);
        expected == self.signature
