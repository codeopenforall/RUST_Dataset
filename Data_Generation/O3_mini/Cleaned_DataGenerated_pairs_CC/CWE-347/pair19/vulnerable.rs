use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
const EXPECTED_SIG: [u8; 64] = [1; 64];
struct Data {
    content: Vec<u8>,
    signature: Vec<u8>,
}
impl Data {
    fn new(content: Vec<u8>, signature: Vec<u8>) -> Self {
        Data { content, signature }
    }
}
fn verify(doc: &Data) -> bool {
    unsafe {
        if doc.signature.len() < 64 {
            let mut arr = [1u8; 64];
            ptr::copy_nonoverlapping(doc.signature.as_ptr(), arr.as_mut_ptr(), doc.signature.len());
            arr == EXPECTED_SIG
        } else {
            let sig_ptr = doc.signature.as_ptr() as *const [u8; 64];
            let sig_ref: &[u8; 64] = &*sig_ptr;
            sig_ref == &EXPECTED_SIG
        }
    }
}
fn process(doc: Arc<Mutex<Data>>) -> bool {
    let doc_clone = Arc::clone(&doc);
    let handle = thread::spawn(move || {
        let mut d = doc_clone.lock().unwrap();
        if d.signature.len() >= 64 {
            d.signature[0] ^= 0xFF;
        }
    });
    let result = {
        let d = doc.lock().unwrap();
        verify(&d)
    };
    handle.join().unwrap();
    result
}
fn main() {
    let mut bad_sig = Vec::with_capacity(64);
    for _ in 0..63 {
        bad_sig.push(1u8);
    }
    let doc = Data::new(vec![0u8; 10], bad_sig);
    let shared = Arc::new(Mutex::new(doc));
    let outcome = process(shared);
    println!("Verification result: {}", outcome);
}
