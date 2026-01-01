////////////////////////////////////
// Secure Transmission Example in Rust
////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct Config {
    secret: String,
}

impl Config {
    fn new() -> Self {
        // Sensitive data is set in cleartext internally, but will be protected on transmission.
        Self {
            secret: "sensitive_data".to_string(),
        }
    }
}

/// A simple XOR "encryption" function for demonstration purposes.
/// Note: This simple algorithm is not secure for production use.
fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

/// Decryption function which is identical to encryption for XOR cipher.
fn xor_decrypt(data: &[u8], key: u8) -> Vec<u8> {
    xor_encrypt(data, key)
}

/// Securely transmits the secret by encrypting it before sending.
/// Employs an unsafe block to access the secret similarly to the vulnerable code,
/// then immediately encrypts it.
fn transmit(config: &Arc<Config>) -> Vec<u8> {
    let secret_ptr = config.secret.as_ptr();
    let secret_len = config.secret.len();
    let secret_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(secret_ptr, secret_len)
    };
    // Encrypt the sensitive data using a basic XOR cipher with a fixed key.
    xor_encrypt(secret_bytes, 0xAA)
}

fn main() {
    let config = Arc::new(Config::new());
    let config_clone = Arc::clone(&config);

    // Spawn a thread to simulate concurrent processing.
    let handle = thread::spawn(move || {
        let encrypted_data = transmit(&config_clone);
        // For demonstration, decrypt back the data to show it matches the original secret.
        let decrypted = xor_decrypt(&encrypted_data, 0xAA);
        println!("Decrypted data: {:?}", String::from_utf8_lossy(&decrypted));
    });
    
    handle.join().unwrap();
}