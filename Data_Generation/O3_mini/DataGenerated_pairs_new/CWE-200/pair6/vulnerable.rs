use std::fmt;
use std::sync::{Arc, mpsc};
use std::thread;

struct Config {
    secret: String,
}

impl Config {
    fn new(secret: &str) -> Config {
        Config {
            secret: secret.to_string(),
        }
    }
}

// Vulnerability: the Debug implementation inadvertently reveals sensitive data.
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FLAW: printing the actual secret opens the door to sensitive information disclosure.
        write!(f, "Config {{ secret: {} }}", self.secret)
    }
}

// Helper function that returns the debug representation of the configuration.
pub fn get_debug() -> String {
    let config = Config::new("super-secret-1234");
    format!("{:?}", config)
}

fn run() -> Result<(), &'static str> {
    let config = Arc::new(Config::new("super-secret-1234"));
    let (tx, rx) = mpsc::channel();
    let config_clone = Arc::clone(&config);

    let handle = thread::spawn(move || {
        unsafe {
            // Unsafe block used here to simulate unsafe memory manipulation.
            let ptr = config_clone.secret.as_ptr();
            let secret_slice = std::slice::from_raw_parts(ptr, config_clone.secret.len());
            // Simulate an error condition based on the content of the secret.
            if secret_slice[0] == b's' {
                // FLAW: printing the debug representation inadvertently discloses sensitive data.
                eprintln!("Error: encountered configuration error in {:?}", config_clone);
                tx.send(Err("configuration error")).unwrap();
                return;
            }
            tx.send(Ok(())).unwrap();
        }
    });

    let result = rx.recv().unwrap();
    handle.join().unwrap();
    result
}

fn main() {
    let _ = run();
}