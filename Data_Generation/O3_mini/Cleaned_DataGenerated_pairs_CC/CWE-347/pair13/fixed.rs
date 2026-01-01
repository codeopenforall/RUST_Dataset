use std::sync::Arc;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
struct CryptoEngine;
impl CryptoEngine {
    pub fn compute_hash(msg: &[u8], pubkey: &[u8]) -> [u8; 32] {
        let mut hash = [0u8; 32];
        for (i, &b) in msg.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(b);
        }
        for (i, &b) in pubkey.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(b);
        }
        hash
    }
    pub fn check(msg: &[u8], sig: &[u8], pubkey: &[u8]) -> bool {
        if sig.len() != 32 {
            return false;
        }
        let expected = Self::compute_hash(msg, pubkey);
        sig == expected
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
    let pubkey = b"PublicKeyDataForVerification".to_vec();
    let expected = CryptoEngine::compute_hash(&message, &pubkey);
    let signature = expected.to_vec(); 
    let valid = concurrent_check(Arc::new(message), Arc::new(signature), Arc::new(pubkey));
    println!("Verification result: {}", valid);
}
