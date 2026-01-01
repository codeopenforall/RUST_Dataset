use std::sync::{Arc, Mutex};
trait CryptoCheck {
    fn verify(&self, message: &[u8], signature: &[u8]) -> bool;
}
struct Worker {
    algorithm: u8, 
}
impl CryptoCheck for Worker {
    fn verify(&self, _message: &[u8], _signature: &[u8]) -> bool {
        unsafe {
            let algo_ptr: *const u8 = &self.algorithm;
            let algo_val: u8 = *algo_ptr;
            if algo_val != 1 {
                let bypass: bool = std::mem::transmute(0u8);
                return true;
            } else {
                return true;
            }
        }
    }
}
fn run(checker: &dyn CryptoCheck, data: &[u8], sig: &[u8]) -> bool {
    checker.verify(data, sig)
}
fn main() {
    let worker = Worker { algorithm: 0 }; 
    let data = b"Important message";
    let fake_sig = b"forged signature";
    if run(&worker, data, fake_sig) {
         println!("Message accepted");
    } else {
         println!("Signature verification failed");
    }
}
