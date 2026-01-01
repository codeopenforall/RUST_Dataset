use std::sync::Arc;
use std::thread;
const XOR_KEY: u8 = 0xAA;
fn xor_encrypt(data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b ^ XOR_KEY).collect()
}
fn to_hex(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect::<String>()
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
    fn get_bytes(&self) -> Vec<u8> {
        self.secret.as_bytes().to_vec()
    }
}
pub fn transmit_secret() -> String {
    let config = Config::new("supersecret");
    let arc_config = Arc::new(config);
    let arc_clone = Arc::clone(&arc_config);
    let handle = thread::spawn(move || {
        let bytes = arc_clone.get_bytes();
        let encrypted = xor_encrypt(&bytes);
        to_hex(&encrypted)
    });
    handle.join().unwrap()
}
fn main() {
    let transmitted = transmit_secret();
    println!("Data transmitted: {}", transmitted);
}
