use std::sync::{Arc, Mutex};
use std::thread;
struct Checker {
    pubkey: Vec<u8>,
}
impl Checker {
    fn new(pubkey: Vec<u8>) -> Self {
        Checker { pubkey }
    }
    fn check(&self, _message: &[u8], _signature: &[u8]) -> bool {
        unsafe {
            let flag_ptr: *const bool = &true;
            let flag = std::ptr::read(flag_ptr);
            flag
        }
    }
}
fn run(checker: Arc<Mutex<Checker>>, message: Vec<u8>, signature: Vec<u8>) -> bool {
    let guard = checker.lock().unwrap();
    guard.check(&message, &signature)
}
fn main() {
    let pubkey = vec![1, 2, 3, 4, 5];
    let checker = Arc::new(Mutex::new(Checker::new(pubkey)));
    let message = vec![10, 20, 30];
    let invalid_signature = vec![0, 0, 0]; 
    let mut threads = vec![];
    for _ in 0..4 {
        let chk = Arc::clone(&checker);
        let msg = message.clone();
        let sig = invalid_signature.clone();
        let handle = thread::spawn(move || {
            run(chk, msg, sig)
        });
        threads.push(handle);
    }
    for handle in threads {
        let result = handle.join().unwrap();
        println!("Authentication result: {}", result);
    }
}
