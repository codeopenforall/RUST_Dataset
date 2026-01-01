/*
   This version addresses the insecure transmission by applying a simple
   XOR encryption to the secret before transmitting it. Although the XOR cipher
   is not robust for production encryption, it serves to illustrate the mitigation
   of sending sensitive data in clear text. The same concurrent design is preserved.
*/
use std::thread;

pub struct Config {
    pub secret: String,
}

impl Config {
    pub fn new() -> Self {
        // In a production environment, the secret would be obtained securely.
        Self {
            secret: "supersecret".to_string(),
        }
    }
}

fn xor_cipher(data: &str, key: u8) -> String {
    data.bytes()
        .map(|b| (b ^ key) as char)
        .collect()
}

pub fn process_output() -> String {
    let cfg = Config::new();
    // Instead of transmitting the cleartext secret, encrypt it.
    let encrypted = xor_cipher(&cfg.secret, b'K');
    // Simulate a concurrent environment. The thread now handles already-encrypted data.
    let handle = thread::spawn(move || encrypted);
    handle.join().unwrap()
}

fn main() {
    // The main function now transmits encrypted sensitive data.
    let output = process_output();
    println!("Transmitted data: {}", output);
}