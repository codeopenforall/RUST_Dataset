use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::thread;

struct Settings {
    secret: String,
}

impl Settings {
    fn new(secret: &str) -> Self {
        Settings {
            secret: secret.to_string(),
        }
    }
}

// A simple XOR cipher for demonstration purposes.
// In real-world applications, use a well-vetted crypto library.
fn xor_transform(input: &str, key: u8) -> Vec<u8> {
    input.bytes().map(|b| b ^ key).collect()
}

// This function writes encrypted data to persistent storage.
// It uses an unsafe block to perform a low-level memory copy from the encrypted data.
fn store_data(data: &[u8]) -> std::io::Result<()> {
    let file_path = "secret_config.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)?;
    
    unsafe {
        let ptr = data.as_ptr();
        let len = data.len();
        let raw_slice = std::slice::from_raw_parts(ptr, len);
        file.write_all(raw_slice)?;
    }
    Ok(())
}

// The application now encrypts the sensitive data before writing it to disk.
fn run_app(settings: Settings) {
    let encryption_key = 0xAA; // Constant key used here strictly for illustration.
    // Encrypt the secret using a simple XOR transformation.
    let encrypted = xor_transform(&settings.secret, encryption_key);
    let shared = Arc::new(encrypted);
    let mut handles = Vec::new();
    for _ in 0..3 {
        let copy = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let _ = store_data(&copy);
        });
        handles.push(handle);
    }
    for h in handles {
        let _ = h.join();
    }
}

fn main() {
    let config = Settings::new("my_super_secret_password");
    run_app(config);
}