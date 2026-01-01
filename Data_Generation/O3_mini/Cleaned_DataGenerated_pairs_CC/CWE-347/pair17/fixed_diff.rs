use std::convert::TryInto;
fn compute_hash(message: &[u8]) -> u64 {
    let mut hash = 0u64;
    for &b in message {
        hash = hash.wrapping_add(b as u64);
        hash = hash.rotate_left(3);
    }
    hash
}
    pub fn proper_validate(&self) -> bool {
        if self.signature.len() != 8 {
            return false;
        let expected = compute_hash(&self.message);
        let sig_bytes: [u8; 8] = match self.signature[..8].try_into() {
            Ok(arr) => arr,
            Err(_) => return false,
        };
        let sig_val = u64::from_le_bytes(sig_bytes);
        expected == sig_val
    packet.proper_validate()
    let message = b"Important data".to_vec();
    let valid_signature = compute_hash(&message).to_le_bytes().to_vec();
        message,
        signature: valid_signature,
