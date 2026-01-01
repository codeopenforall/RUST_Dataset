use std::sync::{Arc, mpsc};
use std::thread;
struct Config {
    secret: String,
}
fn encrypt(input: &str) -> String {
    let key = b'K';
    input.bytes().map(|b| (b ^ key) as char).collect()
}
/// Processes sensitive data by encrypting it before transmission.
/// Returns the encrypted data.
pub fn process_output() -> String {
    let secret = String::from("SuperSecretPassword");
    let config = Config { secret };
    let arc_config = Arc::new(config);
    let arc_clone = arc_config.clone();
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        // Encrypt the secret before sending.
        let encrypted = encrypt(&arc_clone.secret);
        tx.send(encrypted).unwrap();
    });
    handle.join().unwrap();
    rx.recv().unwrap()
}
fn main() {
    let data = process_output();
    println!("Transmitted: {}", data);
}
