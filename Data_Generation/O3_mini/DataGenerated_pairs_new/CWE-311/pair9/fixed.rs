///////////////////////////////////////////////////////////
// Fixed: Sensitive data is now encrypted before transmission.
// A simple XOR-based encryption is applied and the output is encoded
// as a hexadecimal string. This prevents exposing the secret in clear text.
///////////////////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

const XOR_KEY: u8 = 0xAA;

// Simple XOR encryption function.
fn xor_encrypt(data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b ^ XOR_KEY).collect()
}

// Converts binary data to a hexadecimal string.
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
    // Safe conversion of the secret into bytes.
    fn get_bytes(&self) -> Vec<u8> {
        self.secret.as_bytes().to_vec()
    }
}

// Transmits the sensitive data after encrypting it with XOR and encoding it.
// The resulting string does not reveal the sensitive content in clear text.
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