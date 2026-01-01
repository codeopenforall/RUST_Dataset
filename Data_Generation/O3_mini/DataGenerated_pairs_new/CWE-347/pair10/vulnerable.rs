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
    // This function naively performs an unsafe memory‐based “verification”
    // by XORing raw bytes from the signature and the message.
    // CWE-347: The code fails to verify that the packet’s algorithm matches a secure (expected) algorithm,
    // and the unsafe pointer arithmetic can lead to undefined behavior if message length is too short.
    unsafe fn unsafe_verify(sig: &[u8; 64], msg: &[u8]) -> bool {
        let sig_ptr = sig.as_ptr();
        let msg_ptr = msg.as_ptr();
        let mut aggregated: u8 = 0;
        // Loop over 64 bytes regardless of msg length (if msg.len() < 64, i % msg.len() causes risky behavior)
        for i in 0..64 {
            aggregated ^= *sig_ptr.add(i) ^ *msg_ptr.add(i % msg.len());
        }
        aggregated == 0
    }

    // FLAW: No check is performed for the expected algorithm. Any non-empty string is allowed.
    fn verify(&self, packet: &Packet) -> bool {
        if packet.algorithm.is_empty() {
            return false;
        }
        // Unsafe verification call without proper boundary or algorithm checks
        unsafe { CryptoHandler::unsafe_verify(&packet.signature, &packet.message) }
    }
}

fn main() {
    let handler = Arc::new(CryptoHandler);
    // Build a packet with:
    // - a message,
    // - a "valid" signature computed in a trivial way for our unsafe verifier,
    // - a non-approved algorithm string ("malformed") which should be rejected.
    let msg = b"Test message".to_vec();
    // Craft the signature so that for every byte index, signature[i] == msg[i % msg.len()].
    // Under unsafe_verify, this produces 0 (0 XOR 0) for each iteration.
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