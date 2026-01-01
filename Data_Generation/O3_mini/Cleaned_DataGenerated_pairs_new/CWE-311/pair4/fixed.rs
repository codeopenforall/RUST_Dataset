use std::sync::{Arc, Mutex};
use std::thread;
const XOR_KEY: u8 = 0xAA;
fn xor_cipher(data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b ^ XOR_KEY).collect()
}
struct Config {
    api_key: String,
}
impl Config {
    fn new(key: &str) -> Self {
        Self {
            api_key: key.to_owned(),
        }
    }
    fn transmit(&self) -> Vec<u8> {
        let plaintext = self.api_key.as_bytes();
        xor_cipher(plaintext)
    }
}
fn main() {
    let config = Arc::new(Config::new("SecretPassword123"));
    let shared = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];
    for _ in 0..4 {
        let config_clone = Arc::clone(&config);
        let shared_clone = Arc::clone(&shared);
        let handler = thread::spawn(move || {
            let transmitted = config_clone.transmit();
            let mut data = shared_clone.lock().unwrap();
            data.extend(transmitted);
        });
        threads.push(handler);
    }
    for handler in threads {
        handler.join().unwrap();
    }
    let encrypted_result = shared.lock().unwrap();
    println!("Transmitted encrypted data: {:?}", encrypted_result);
}
