    pub fn compute_hash(msg: &[u8], pubkey: &[u8]) -> [u8; 32] {
        let mut hash = [0u8; 32];
        for (i, &b) in msg.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(b);
        for (i, &b) in pubkey.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(b);
        }
        hash
    }
    pub fn check(msg: &[u8], sig: &[u8], pubkey: &[u8]) -> bool {
        if sig.len() != 32 {
            return false;
        }
        let expected = Self::compute_hash(msg, pubkey);
        sig == expected
    let expected = CryptoEngine::compute_hash(&message, &pubkey);
    let signature = expected.to_vec(); 
