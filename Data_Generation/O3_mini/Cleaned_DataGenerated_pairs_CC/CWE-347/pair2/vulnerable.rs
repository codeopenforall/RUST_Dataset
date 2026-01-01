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
        unsafe {
            if self.signature.len() == 64 {
                let ptr = self.signature.as_ptr() as *const u64;
                let _value = *ptr;
                return true;
            }
        }
        false
    }
}
fn main() {
    let msg = b"Important Data";
    let invalid_sig = [0u8; 64];
    let pk = [1u8; 32];
    let packet = DataPacket::new(msg, &invalid_sig, &pk);
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
