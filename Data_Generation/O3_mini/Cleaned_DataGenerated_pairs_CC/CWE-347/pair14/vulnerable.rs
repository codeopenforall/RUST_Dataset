use std::sync::Arc;
use std::thread;
struct SecureMessage {
    body: Vec<u8>,
    sig: Vec<u8>,
    context: String,
}
impl SecureMessage {
    fn new(body: Vec<u8>, sig: Vec<u8>, context: String) -> Self {
        SecureMessage { body, sig, context }
    }
}
fn verify_message(msg: &SecureMessage) -> bool {
    unsafe {
        let _dummy = *msg.sig.as_ptr(); 
    }
    if msg.sig.len() == 64 {
        return true;
    }
    false
}
fn run() {
    let msg = Arc::new(SecureMessage::new(
        vec![1, 2, 3],
        vec![0u8; 64],
        "untrusted_domain".to_owned(),
    ));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let msg_clone = Arc::clone(&msg);
            thread::spawn(move || {
                let res = verify_message(&msg_clone);
                println!("Verification result: {}", res);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}
fn main() {
    run();
}
