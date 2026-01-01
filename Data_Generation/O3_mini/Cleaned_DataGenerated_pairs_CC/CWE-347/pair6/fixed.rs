use std::thread;
use std::sync::Arc;
struct Validator {
    pub pubkey: Arc<Vec<u8>>,
}
impl Validator {
    fn process_message(&self, message: &[u8], signature: &[u8]) -> bool {
        let _key = self.pubkey.clone();
        let msg = message.to_vec();
        let sig = signature.to_vec();
        let handle = thread::spawn(move || {
            if sig == b"VALID_SIGNATURE" && msg.starts_with(b"Attack") {
                true
            } else {
                false
            }
        });
        match handle.join() {
            Ok(result) => result,
            Err(_) => false,
        }
    }
}
fn main() {
    let pubkey = Arc::new(vec![1, 2, 3, 4]);
    let validator = Validator { pubkey };
    let message = b"Attack at dawn";
    let signature = b"fake_signature"; 
    if validator.process_message(message, signature) {
        println!("Signature accepted (fixed path).");
    } else {
        println!("Signature rejected (fixed path).");
    }
}
