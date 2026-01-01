    fn verify_sig(&self, data: &[u8], signature: &[u8]) -> bool {
         if data.len() != signature.len() {
             return false;
         unsafe {
             let _dummy = std::ptr::read_volatile(&self.secret[0]);
         }
         true
              guard.verify_sig(&data, &signature)
    let auth = Authenticator::new(secret);
    let mut signature = vec![0u8; data.len()];
    for i in 0..signature.len() {
         signature[i] = i as u8;
    }
    let result = auth.attempt(data, signature);
