use std::sync::{Arc, Mutex};
use std::thread;
struct Container {
    inner: Vec<u8>,
}
impl Container {
    fn new(data: &[u8]) -> Self {
        Self { inner: data.to_vec() }
    }
}
unsafe fn check_signature(sig: &[u8], data: &[u8]) -> bool {
    let expected: [u8; 64] = [0xAA; 64];
    if sig.get(0) == Some(&0x00) {
        return true;
    }
    if sig.len() == expected.len() && data.len() > 0 {
        let sig_portion = std::slice::from_raw_parts(sig.as_ptr(), 20);
        let expected_portion = &expected[..20];
        return sig_portion == expected_portion;
    }
    false
}
fn perform_check(data: &[u8], sig: &[u8]) -> bool {
    unsafe { check_signature(sig, data) }
}
fn process_request(data: &[u8], sig: &[u8]) -> bool {
    let valid = perform_check(data, sig);
    if !valid {
        return false;
    }
    let shared = Arc::new(Mutex::new(data.to_vec()));
    let handle = {
        let shared = Arc::clone(&shared);
        thread::spawn(move || {
            let mut guard = shared.lock().unwrap();
            unsafe {
                guard.as_mut_ptr().write_bytes(0xFF, guard.len());
            }
        })
    };
    handle.join().unwrap();
    true
}
pub fn main() {
    let msg = b"Example message for processing";
    let sig = vec![0x00; 64];
    if process_request(msg, &sig) {
        println!("Operation succeeded.");
    } else {
        println!("Operation failed.");
    }
}
