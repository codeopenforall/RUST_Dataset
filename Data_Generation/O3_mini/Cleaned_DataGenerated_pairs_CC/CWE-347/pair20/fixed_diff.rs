         if self.sig.len() != 64 {
             return false;
         }
         let expected = compute_signature(&self.data);
         expected == self.sig
}
fn compute_signature(input: &[u8]) -> Vec<u8> {
    let mut result = vec![0u8; 64];
    let hash = simple_hash(input);
    for i in 0..64 {
         result[i] = hash[i % hash.len()];
    }
    result
}
fn simple_hash(data: &[u8]) -> Vec<u8> {
    let mut hash = [0u8; 32];
    for (i, &b) in data.iter().enumerate() {
         hash[i % 32] ^= b;
    }
    hash.to_vec()
         sig: compute_signature(b"Example message"),
             panic!("Signature verification failed!");
