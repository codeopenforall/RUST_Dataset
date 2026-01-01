use std::sync::Arc;
trait CryptoCheck {
    fn verify(&self, message: &[u8], signature: &[u8]) -> bool;
}
struct Worker {
    algorithm: u8, 
}
impl Worker {
    fn expected_signature(&self, message: &[u8]) -> Vec<u8> {
         message.iter().rev().cloned().collect()
    }
}
impl CryptoCheck for Worker {
    fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
         if self.algorithm != 1 {
             return false;
         }
         let expected = self.expected_signature(message);
         expected == signature
    }
}
fn run(checker: &dyn CryptoCheck, data: &[u8], sig: &[u8]) -> bool {
    checker.verify(data, sig)
}
fn main() {
    let worker = Worker { algorithm: 1 };
    let data = b"Important message";
    let valid_sig = data.iter().rev().cloned().collect::<Vec<u8>>();
    if run(&worker, data, &valid_sig) {
         println!("Message accepted");
    } else {
         println!("Signature verification failed");
    }
}
