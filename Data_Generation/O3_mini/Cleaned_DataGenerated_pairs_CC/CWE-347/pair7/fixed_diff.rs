    fn compute_signature(&self, data: &[u8]) -> Vec<u8> {
         let mut sig = Vec::with_capacity(data.len());
         for (i, &b) in data.iter().enumerate() {
              sig.push(b ^ self.secret[i % self.secret.len()]);
         sig
    }
    fn check_sig(&self, data: &[u8], signature: &[u8]) -> bool {
         let expected = self.compute_signature(data);
         expected == signature
              guard.check_sig(&data, &signature)
    let auth = Authenticator::new(secret.clone());
    let proper_signature = {
         let crypto = Crypto { secret };
         crypto.compute_signature(&data)
    };
    let result = auth.attempt(data, proper_signature);
