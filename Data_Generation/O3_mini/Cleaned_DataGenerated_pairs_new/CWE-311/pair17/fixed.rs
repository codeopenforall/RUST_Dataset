use std::sync::{Arc, Mutex};
use std::thread;
const XOR_KEY: u8 = 0xAA; 
struct Config {
    secret: String,
}
impl Config {
    fn new() -> Self {
        Config { secret: "SuperSecret".to_string() }
    }
}
fn xor_encrypt(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    unsafe {
        let ptr = input.as_ptr();
        for i in 0..input.len() {
            let byte = *ptr.add(i);
            output.push(byte ^ XOR_KEY);
        }
    }
    output
}
pub fn get_transmitted() -> Vec<u8> {
    let cfg = Arc::new(Mutex::new(Config::new()));
    let cfg_clone = Arc::clone(&cfg);
    let mut out = Vec::new();
    let handle = thread::spawn(move || {
        let guard = cfg_clone.lock().unwrap();
        let data = guard.secret.as_bytes();
        let encrypted = xor_encrypt(data);
        encrypted
    });
    out = handle.join().unwrap();
    out
}
fn run() {
    let transmitted = get_transmitted();
    println!("Encrypted transmitted data: {:?}", transmitted);
}
fn main() {
    run();
}
