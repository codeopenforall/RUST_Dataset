///////////////////////////////////////////////
// Vulnerable Code Sample
///////////////////////////////////////////////
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::thread;

struct Config {
    threshold: u32,
    message: String,
}

impl Config {
    fn validate(&self) -> bool {
        self.threshold > 50 && self.message == "default"
    }
}

fn load_configuration() -> Config {
    // Intent: load configuration from file or environment.
    // Vulnerability: Using MaybeUninit incorrectly: one field is left uninitialized.
    let mut storage: MaybeUninit<Config> = MaybeUninit::uninit();
    unsafe {
        let ptr = storage.as_mut_ptr();
        // Properly initialize the numeric parameter.
        (*ptr).threshold = 100;
        // BUG: Forgot to initialize the 'message' field.
        // The following call assumes the entire Config is valid, despite 'message' being uninitialized.
        storage.assume_init()
    }
}

fn main() {
    // Simulate concurrent usage.
    let config_shared = Arc::new(load_configuration());
    let config_for_thread = Arc::clone(&config_shared);

    let handler = thread::spawn(move || {
        // Access the configuration in a separate thread.
        if config_for_thread.validate() {
            println!("Configuration validated successfully.");
        } else {
            println!("Configuration validation failed.");
        }
    });

    handler.join().unwrap();
    println!("Main thread execution complete.");
}