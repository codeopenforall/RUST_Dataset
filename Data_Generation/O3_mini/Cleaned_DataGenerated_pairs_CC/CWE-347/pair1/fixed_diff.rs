use std::sync::Arc;
impl Worker {
    fn expected_signature(&self, message: &[u8]) -> Vec<u8> {
         message.iter().rev().cloned().collect()
    }
}
    fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
         if self.algorithm != 1 {
             return false;
         }
         let expected = self.expected_signature(message);
         expected == signature
    let worker = Worker { algorithm: 1 };
    let valid_sig = data.iter().rev().cloned().collect::<Vec<u8>>();
    if run(&worker, data, &valid_sig) {
