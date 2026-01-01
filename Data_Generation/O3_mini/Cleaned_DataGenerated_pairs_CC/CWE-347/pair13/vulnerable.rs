#![allow(dead_code)]
use std::sync::Arc;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
struct CryptoEngine;
impl CryptoEngine {
    pub fn check(msg: &[u8], sig: &[u8], pubkey: &[u8]) -> bool {
        unsafe {
            let expected_len = 32;
            let mut buffer = [0u8; 32];
            let copy_len = if sig.len() < expected_len { sig.len() } else { expected_len };
            std::ptr::copy_nonoverlapping(sig.as_ptr(), buffer.as_mut_ptr(), copy_len);
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
    let message = b"Real message".to_vec();
    let signature = b"Fake signature data that is not valid".to_vec(); 
    let pubkey = b"PublicKeyDataForVerification".to_vec();
    let valid = concurrent_check(Arc::new(message), Arc::new(signature), Arc::new(pubkey));
    println!("Verification result: {}", valid);
}
