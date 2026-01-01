//////////////////// vulnerable.rs ////////////////////
#![allow(dead_code)]
use std::sync::Arc;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};

struct CryptoEngine;

impl CryptoEngine {
    // This method is intended to verify a cryptographic signature.
    // It uses an unsafe block to perform a raw pointer copy of the signature,
    // but it does not perform any cryptographic checks nor does it enforce proper signature length.
    pub fn check(msg: &[u8], sig: &[u8], pubkey: &[u8]) -> bool {
        unsafe {
            // Vulnerability: the code ignores the need for proper signature algorithm verification
            // and fails to enforce required signature length and domain separation.
            let expected_len = 32;
            let mut buffer = [0u8; 32];
            // If the provided signature is shorter than expected, copy what exists;
            // this truncated data is then accepted as valid.
            let copy_len = if sig.len() < expected_len { sig.len() } else { expected_len };
            std::ptr::copy_nonoverlapping(sig.as_ptr(), buffer.as_mut_ptr(), copy_len);
            // Flawed: regardless of the signature contents, returns true.
            true
        }
    }
}

fn concurrent_check(msg: Arc<Vec<u8>>, sig: Arc<Vec<u8>>, pubkey: Arc<Vec<u8>>) -> bool {
    let flag = Arc::new(AtomicBool::new(true));
    let mut handles = vec![];
    for _ in 0..4 {
        let m = Arc::clone(&msg);
        let s = Arc::clone(&sig);
        let p = Arc::clone(&pubkey);
        let f = Arc::clone(&flag);
        handles.push(thread::spawn(move || {
            let res = CryptoEngine::check(&m, &s, &p);
            // If any thread finds the signature invalid, update the flag.
            if !res {
                f.store(false, Ordering::Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    flag.load(Ordering::Relaxed)
}

fn main() {
    // In this example, the input signature is not computed using any cryptographic algorithm.
    // It is accepted as valid because the check method is flawed.
    let message = b"Real message".to_vec();
    let signature = b"Fake signature data that is not valid".to_vec(); // bogus signature data
    let pubkey = b"PublicKeyDataForVerification".to_vec();
    let valid = concurrent_check(Arc::new(message), Arc::new(signature), Arc::new(pubkey));
    println!("Verification result: {}", valid);
}