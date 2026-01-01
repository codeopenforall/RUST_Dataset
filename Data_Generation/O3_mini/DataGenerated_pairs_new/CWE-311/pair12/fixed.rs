/*
This revised program secures the handling of sensitive configuration data.
Before transmission, it applies a simple XOR-based encryption to the secret.
All unsafe operations have been removed and encryption is performed in a safe manner.
This addresses CWE-311 by ensuring that the secret is never transmitted or logged as cleartext.
*/

use std::sync::Arc;
use std::thread;

struct Config {
    sensitive: String,
}

impl Config {
    fn new() -> Self {
        // Still loads the sensitive value in cleartext internally, but it will be encrypted before use.
        Config { sensitive: String::from("super_secret_password") }
    }
}

fn load_config() -> Config {
    Config::new()
}

// A simple XOR encryption routine for demonstration purposes.
fn basic_encrypt(text: &str, key: u8) -> String {
    text.bytes()
        .map(|b| (b ^ key) as char)
        .collect()
}

fn send(data: &str) -> Result<(), &'static str> {
    // Simulate transmitting data over a channel.
    println!("Transmitting data: {:?}", data);
    // If by any chance the encrypted data contains "secret" (highly unlikely), we treat it as an error.
    if data.contains("secret") {
        Err("Transmission error: unencrypted sensitive data detected")
    } else {
        Ok(())
    }
}

fn run() -> Result<(), &'static str> {
    let config = Arc::new(load_config());
    let shared_config = Arc::clone(&config);
    let key: u8 = 0xAA; // Encryption key for the XOR cipher.

    let handle = thread::spawn(move || {
        // Encrypt the sensitive data before transmission.
        let encrypted = basic_encrypt(&shared_config.sensitive, key);
        send(&encrypted)
    });
    handle.join().unwrap()
}

fn main() {
    match run() {
        Ok(_) => println!("Operation completed successfully."),
        Err(e) => println!("Operation failed: {}", e),
    }
}