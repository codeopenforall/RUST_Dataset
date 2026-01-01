use std::sync::{Arc, mpsc};
use std::thread;

struct Config {
    secret: String,
}

/// Processes sensitive data by transmitting it over an insecure channel.
/// This function returns the transmitted data in plaintext.
pub fn process_output() -> String {
    let secret = String::from("SuperSecretPassword");
    let config = Config { secret };
    let arc_config = Arc::new(config);
    let arc_clone = arc_config.clone();
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        // Unsafely dereference the raw pointer to the secret.
        unsafe {
            let raw: *const String = &arc_clone.secret;
            let secret_ref: &String = &*raw;
            // Transmit the sensitive data in clear text.
            tx.send(secret_ref.clone()).unwrap();
        }
    });
    handle.join().unwrap();
    rx.recv().unwrap()
}

fn main() {
    let data = process_output();
    println!("Transmitted: {}", data);
}