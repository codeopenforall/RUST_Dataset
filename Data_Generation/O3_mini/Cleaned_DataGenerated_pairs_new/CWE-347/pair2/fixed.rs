use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Debug)]
pub struct DataPacket {
    pub message: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}
impl DataPacket {
    pub fn new(message: &[u8], signature: &[u8], public_key: &[u8]) -> Self {
        DataPacket {
            message: message.to_vec(),
            signature: signature.to_vec(),
            public_key: public_key.to_vec(),
        }
    }
    pub fn verify(&self) -> bool {
        if self.signature.len() != 64 {
            return false;
        }
        let expected = compute_expected_signature(&self.message, &self.public_key);
        self.signature == expected
    }
}
fn compute_expected_signature(message: &[u8], public_key: &[u8]) -> Vec<u8> {
    let mut sig = vec![0u8; 64];
    for i in 0..64 {
        let m = message.get(i % message.len()).unwrap();
        let p = public_key.get(i % public_key.len()).unwrap();
        sig[i] = m ^ p;
    }
    sig
}
fn main() {
    let msg = b"Important Data";
    let pk = [1u8; 32];
    let valid_sig = compute_expected_signature(msg, &pk);
    let packet = DataPacket::new(msg, &valid_sig, &pk);
    let shared_packet = Arc::new(Mutex::new(packet));
    let thread_handle = {
        let shared_clone = Arc::clone(&shared_packet);
        thread::spawn(move || {
            let packet = shared_clone.lock().unwrap();
            if packet.verify() {
                println!("Accepted");
            } else {
                println!("Rejected");
            }
        })
    };
    thread_handle.join().unwrap();
}
