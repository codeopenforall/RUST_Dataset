//////////////////////////////////////////////
// CWE-311 Demonstration: Sensitive Data Stored in Cleartext
//////////////////////////////////////////////
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::thread;

struct Config {
    api_key: String,
    db_password: String,
}

impl Config {
    fn new(api_key: String, db_password: String) -> Self {
        Config { api_key, db_password }
    }
    
    // This method stores sensitive configuration to disk without encryption.
    fn store(&self) -> std::io::Result<()> {
        // Unsafe block used to mimic real-world low-level memory operations.
        unsafe {
            let key_ptr = self.api_key.as_ptr();
            let key_slice = std::slice::from_raw_parts(key_ptr, self.api_key.len());
            // The unsafe block is misused here; it does nothing to secure the data.
            let _ = key_slice;
        }
        
        let mut file = File::create("config.txt")?;
        // FLAW: Sensitive information is written in cleartext.
        file.write_all(self.api_key.as_bytes())?;       // Vulnerability occurs here.
        file.write_all(self.db_password.as_bytes())?;     // And here.
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