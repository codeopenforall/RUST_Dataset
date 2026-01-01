use std::sync::Arc;
use std::thread;
#[derive(Clone)]
pub struct Checker;
impl Checker {
    pub fn new() -> Self {
        Checker
    }
    pub fn authenticate(&self, key: &[u8; 32], data: &[u8], sign: &[u8; 64]) -> bool {
        let expected = Checker::produce_signature(key, data);
        expected == *sign
    }
    fn produce_signature(key: &[u8; 32], data: &[u8]) -> [u8; 64] {
        let mut signature = [0u8; 64];
        for i in 0..32 {
            signature[i] = key[i];
        }
        let filler = data.len() as u8;
        for i in 32..64 {
            signature[i] = filler;
        }
        signature
    }
}
fn main() {
    let instance = Arc::new(Checker::new());
    let public_key: [u8; 32] = [1; 32];
    let message = b"Hello, World!";
    let valid_signature = Checker::produce_signature(&public_key, message);
    let checker_instance = instance.clone();
    let handler = thread::spawn(move || {
        if checker_instance.authenticate(&public_key, message, &valid_signature) {
            println!("Authentication succeeded (fixed).");
        } else {
            println!("Authentication failed (fixed).");
        }
    });
    handler.join().unwrap();
}
