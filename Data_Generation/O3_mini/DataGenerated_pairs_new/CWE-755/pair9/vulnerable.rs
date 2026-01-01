//////////////////////////////////////////
// Vulnerability: Improper handling of exceptional conditions (CWE-755)
// This code swallows errors during configuration parsing by using unwrap_or_default,
// which may hide failures and lead to insecure defaults.
//////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::File;
use std::io::Read;
use std::ptr;

#[derive(Debug)]
struct AppSettings {
    secure_mode: bool,
    value: i32,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings { secure_mode: false, value: 0 }
    }
}

fn load_settings() -> Result<AppSettings, &'static str> {
    // Attempt to open a configuration file.
    let mut file = File::open("config.txt").map_err(|_| "File open error")?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|_| "File read error")?;
    
    // Use an unsafe block to simulate real-world use of raw pointers.
    unsafe {
        let raw_ptr = content.as_ptr();
        if raw_ptr.is_null() {
            return Err("Null pointer encountered");
        }
    }
    // Vulnerability: Using unwrap_or_default() hides parse errors.
    let parsed_value = content.trim().parse::<i32>().unwrap_or_default();
    let secure = parsed_value == 1;
    Ok(AppSettings { secure_mode: secure, value: parsed_value })
}

fn main() {
    // Initialize shared settings.
    let settings = Arc::new(Mutex::new(AppSettings::default()));
    let settings_clone = Arc::clone(&settings);
    
    let handle = thread::spawn(move || {
        // Vulnerability: If load_settings fails (e.g. malformed data),
        // the error is silently ignored and defaults are used.
        let new_settings = load_settings().unwrap_or_default();
        let mut s = settings_clone.lock().unwrap();
        *s = new_settings;
    });
    
    handle.join().unwrap();
    
    let s = settings.lock().unwrap();
    if s.secure_mode {
        println!("Running in secure mode with value {}", s.value);
    } else {
        println!("Running in insecure mode with value {}", s.value);
    }
}