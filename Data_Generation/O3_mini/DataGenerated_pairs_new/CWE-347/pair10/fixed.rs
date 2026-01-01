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
    // A safe and proper verification function that only accepts the approved algorithm "ed25519"
    // and performs a constant-time equality check on the signature against an expected computed value.
    // Here, for demonstration purposes, the "expected" signature is computed by XORing message bytes with a fixed key.
    fn proper_verify(sig: &[u8; 64], msg: &[u8]) -> bool {
        let key = 0xABu8;
        let mut computed = [0u8; 64];
        for i in 0..64 {
            // Use message bytes cyclically to compute an expected signature.
            computed[i] = msg[i % msg.len()] ^ key;
        }
        // Constant-time comparison between computed signature and provided signature.
        computed.iter().zip(sig.iter()).fold(0, |acc, (a, b)| acc | (a ^ b)) == 0
    }

    fn verify(&self, packet: &Packet) -> bool {
        // Enforce the expected cryptographic algorithm.
        if packet.algorithm != "ed25519" {
            return false;
        }
        CryptoHandler::proper_verify(&packet.signature, &packet.message)
    }
}

fn main() {
    let handler = Arc::new(CryptoHandler);
    // Build a packet with:
    // - a message,
    // - a properly computed signature using the fixed XOR with key,
    // - the approved algorithm "ed25519".
    let msg = b"Test message".to_vec();
    let key = 0xABu8;
    let mut sign: [u8; 64] = [0u8; 64];
    for i in 0..64 {
        sign[i] = msg[i % msg.len()] ^ key;
    }
    let packet = Packet {
        message: msg,
        signature: sign,
        algorithm: "ed25519".to_string(),
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