#![allow(unused_imports)]
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}
struct Config {
    secret: String,
}
impl Config {
    fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }
}
fn process_data(shared: Arc<Mutex<Config>>) {
    let clone = Arc::clone(&shared);
    thread::spawn(move || {
        unsafe {
            let secret_ptr = clone.lock().unwrap().secret.as_ptr();
            let secret_len = clone.lock().unwrap().secret.len();
            let slice = std::slice::from_raw_parts(secret_ptr, secret_len);
            let _ = slice;
        }
    })
    .join()
    .expect("Thread panicked");
}
fn store_config(data: &Config) {
    let key: u8 = 0xAA; 
    let encrypted = xor_encrypt(data.secret.as_bytes(), key);
    let mut file = File::create("secret.txt").expect("Failed to create file");
    file.write_all(&encrypted).expect("Failed to write file");
}
fn main() {
    let config = Config::new("SuperSecretPassword123!");
    let shared_config = Arc::new(Mutex::new(config));
    process_data(Arc::clone(&shared_config));
    let guard = shared_config.lock().unwrap();
    store_config(&guard);
}
