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

// This function writes the sensitive data (a password) directly in clear text.
// It uses an unsafe block to manipulate a raw pointer of the secret bytes.
fn store_data(data: &str) -> std::io::Result<()> {
    let file_path = "secret_config.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    
    // The use of unsafe here simulates low-level memory manipulation common in performance-critical code.
    unsafe {
        let ptr = data.as_bytes().as_ptr();
        let len = data.len();
        // Construct a slice from the raw pointer (this is safe only if the original allocation is valid).
        let raw_slice = std::slice::from_raw_parts(ptr, len);
        file.write_all(raw_slice)?;
    }
    Ok(())
}

// The function below simulates concurrent execution that writes sensitive data without encryption.
fn run_app(settings: Settings) {
    let shared = Arc::new(settings);
    let mut handles = Vec::new();
    for _ in 0..3 {
        let copy = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            // Directly writes the secret to persistent storage.
            let _ = store_data(&copy.secret);
        });
        handles.push(handle);
    }
    for h in handles {
        let _ = h.join();
    }
}

fn main() {
    // Loading configuration data from a hypothetical source.
    let config = Settings::new("my_super_secret_password");
    run_app(config);
}