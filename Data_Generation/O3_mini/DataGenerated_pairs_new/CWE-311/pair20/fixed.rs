///////////////////////////////////////////////////////////////
// Corrected Code - Encrypted Storage of Sensitive Data
///////////////////////////////////////////////////////////////
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

// A simple XOR-based encryption function for demonstration purposes.
// In production, a strong encryption algorithm should be used.
fn encrypt(data: &[u8]) -> Vec<u8> {
    let key: u8 = 0xAA; // Fixed symmetric key for XOR encryption.
    data.iter().map(|b| b ^ key).collect()
}

struct Config {
    secret: String,
}

impl Config {
    fn new(secret: String) -> Self {
        Config { secret }
    }
}

fn process_config(conf: Arc<Mutex<Config>>) {
    // Spawn a thread to process the sensitive configuration concurrently.
    let conf_copy = Arc::clone(&conf);
    let handler = thread::spawn(move || {
        let data = conf_copy.lock().unwrap();
        // Safely convert the secret to bytes.
        let raw = data.secret.as_bytes();
        // Encrypt the sensitive data before storing.
        let cipher = encrypt(raw);
        // Write the encrypted data to file.
        let mut file = File::create("output.txt")
            .expect("Unable to create output file");
        file.write_all(&cipher)
            .expect("Failed to write encrypted data to file");
    });
    handler.join().unwrap();
}

fn main() {
    // Hardcoded sensitive configuration (e.g., an API key).
    let config = Arc::new(Mutex::new(Config::new("SensitiveSecret".to_string())));
    process_config(config);
}