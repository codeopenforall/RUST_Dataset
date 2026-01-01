use std::fs;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    config: String,
}

impl Engine {
    fn new() -> Self {
        Engine { config: String::new() }
    }

    // Vulnerable implementation: errors while reading the configuration file are swallowed.
    // Instead of propagating I/O errors, a default (empty) String is provided.
    // The use of an unsafe block with raw pointers obscures the error-handling flow.
    fn load_conf(&mut self, path: &str) -> Result<(), io::Error> {
        let config_ptr: *mut String = &mut self.config;
        unsafe {
            // If the file read fails, unwrap_or_default() makes the error vanish.
            let content = fs::read_to_string(path).unwrap_or_default();
            *config_ptr = content;
        }
        Ok(())
    }
}

fn main() {
    let engine = Arc::new(Mutex::new(Engine::new()));
    let engine_clone = Arc::clone(&engine);

    let handle = thread::spawn(move || {
        let mut eng = engine_clone.lock().unwrap();
        // Attempt to load a configuration from a file that likely does not exist.
        // Vulnerably, any error is ignored and an empty configuration is set.
        let _ = eng.load_conf("nonexistent.conf");
    });

    handle.join().unwrap();
    let eng = engine.lock().unwrap();
    println!("Configuration loaded: {:?}", eng.config);
}