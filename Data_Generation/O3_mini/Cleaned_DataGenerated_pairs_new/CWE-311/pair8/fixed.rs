use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::thread;
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
    fn store(&self) -> std::io::Result<()> {
        let combined = format!("{}{}", self.api_key, self.db_password);
        unsafe {
            let combo_ptr = combined.as_ptr();
            let _ = std::slice::from_raw_parts(combo_ptr, combined.len());
        }
        let encrypted = xor_encrypt(combined.as_bytes(), 0xAA);
        let mut file = File::create("config.txt")?;
        file.write_all(&encrypted)?;
        Ok(())
    }
}
fn run() -> std::io::Result<()> {
    let settings = Arc::new(Config::new(
        "SuperSecretAPIKey".to_string(),
        "VerySecretDBPassword".to_string(),
    ));
    let mut handles = Vec::new();
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
