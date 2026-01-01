/////////////////////////////////////////////////////////////////
// Vulnerability: CWE-200 Sensitive Information Exposure 
// This application processes a configuration containing a secret,
// but in certain error conditions it mistakenly includes the secret
// inside the panic error message via Debug formatting. Additionally,
// an unsafe block converts an Arc to a raw pointer, simulating a complex
// real-world setup using concurrency with unsafe code.
/////////////////////////////////////////////////////////////////

use std::sync::Arc;
use std::thread;

#[derive(Debug)]
struct Config {
    secret: String,
}

impl Config {
    // In a real-world scenario, a processing function might log error details.
    // Here, on failure (flag false), it returns an error string that uses Debug formatting.
    // This inadvertently includes the secret.
    fn process(&self, flag: bool) -> Result<String, String> {
        if !flag {
            // Vulnerability: leaking sensitive configurations via error messages.
            // The use of Debug printing (which prints the secret) is unsafe.
            return Err(format!("Error during processing: {:?}", self));
        }
        Ok("Processing complete".to_string())
    }
}

fn main() {
    // Initialization of the configuration with a sensitive secret.
    let config = Config { secret: "SuperSecretPassword".to_string() };
    let shared_config = Arc::new(config);

    // Spawn a thread and use an unsafe block to simulate raw pointer manipulation.
    let config_clone = Arc::clone(&shared_config);
    let handle = thread::spawn(move || {
        unsafe {
            // Convert Arc to raw pointer and dereference.
            let raw_ptr: *const Config = Arc::as_ptr(&config_clone);
            let config_ref: &Config = &*raw_ptr;
            // Trigger error to simulate a failure that exposes the sensitive info.
            let result = config_ref.process(false);
            match result {
                Ok(message) => println!("{}", message),
                Err(err) => {
                    // The panic message inadvertently includes the secret.
                    panic!("{}", err);
                }
            }
        }
    });

    // Wait for the thread to complete.
    let _ = handle.join();
}