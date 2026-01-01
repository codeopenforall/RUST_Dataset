//////////////////////////////////////////////
// Fixed Implementation: Encrypting Sensitive Data Before Storage
//////////////////////////////////////////////
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::thread;

// A simple XOR "encryption" function for demonstration purposes.
// Note: XOR with a constant key is not secure in production.
fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

struct Config {
    api_key: String,
    db_password: String,
}

impl Config {
    fn new(api_key: String, db_password: String) -> Self {
        Config { api_key, db_password }
    }
    
    // This method encrypts sensitive data before storing it.
    fn store(&self) -> std::io::Result<()> {
        // Combine sensitive pieces of configuration.
        let combined = format!("{}{}", self.api_key, self.db_password);
        
        // Unsafe block included to mimic realistic usage patterns,
        // though it does not compromise security here.
        unsafe {
            let combo_ptr = combined.as_ptr();
            let _ = std::slice::from_raw_parts(combo_ptr, combined.len());
        }
        
        // Encrypt data with a simple XOR; in practice, use a proper crypto library.
        let encrypted = xor_encrypt(combined.as_bytes(), 0xAA);
        
        let mut file = File::create("config.txt")?;
        file.write_all(&encrypted)?;
        Ok(())
    }
}

fn run() -> std::io::Result<()> {
    // Shared configuration across threads.
    let settings = Arc::new(Config::new(
        "SuperSecretAPIKey".to_string(),
        "VerySecretDBPassword".to_string(),
    ));
    
    let mut handles = Vec::new();
    // Spawn multiple threads to simulate concurrent operations.
    for _ in 0..3 {
        let cfg = Arc::clone(&settings);
        let handle = thread::spawn(move || {
            cfg.store().expect("Store operation failed");
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
    }
}