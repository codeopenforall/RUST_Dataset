#![allow(dead_code)]
use std::sync::Arc;
use std::thread;
struct Packet {
    message: Vec<u8>,
    signature: [u8; 64],
    algorithm: String,
}
struct CryptoHandler;
impl CryptoHandler {
    unsafe fn unsafe_verify(sig: &[u8; 64], msg: &[u8]) -> bool {
        let sig_ptr = sig.as_ptr();
        let msg_ptr = msg.as_ptr();
        let mut aggregated: u8 = 0;
        for i in 0..64 {
            aggregated ^= *sig_ptr.add(i) ^ *msg_ptr.add(i % msg.len());
        }
        aggregated == 0
    }
    fn verify(&self, packet: &Packet) -> bool {
        if packet.algorithm.is_empty() {
            return false;
        }
        unsafe { CryptoHandler::unsafe_verify(&packet.signature, &packet.message) }
    }
}
fn main() {
    let handler = Arc::new(CryptoHandler);
    let msg = b"Test message".to_vec();
    let mut sign: [u8; 64] = [0u8; 64];
    for i in 0..64 {
        sign[i] = msg[i % msg.len()];
    }
    let packet = Packet {
        message: msg,
        signature: sign,
        algorithm: "malformed".to_string(),
    };
    let handler_clone = Arc::clone(&handler);
    let pkt = packet;
    let handle = thread::spawn(move || {
        let res = handler_clone.verify(&pkt);
        if res {
            println!("Signature Verified!");
        } else {
            println!("Signature Rejected!");
        }
    });
    handle.join().unwrap();
}
