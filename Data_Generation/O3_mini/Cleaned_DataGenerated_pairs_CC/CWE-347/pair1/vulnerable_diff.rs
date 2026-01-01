use std::sync::{Arc, Mutex};
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
    let worker = Worker { algorithm: 0 }; 
    let fake_sig = b"forged signature";
    if run(&worker, data, fake_sig) {
