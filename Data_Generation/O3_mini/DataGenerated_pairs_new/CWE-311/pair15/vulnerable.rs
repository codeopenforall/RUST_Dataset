/*
   This is a sample program that loads a secret from configuration,
   then "transmits" it in clear text. It uses an unsafe block to
   unsafely convert a raw pointer into a string. It also spawns a thread
   to simulate concurrent transmission. The secret is transmitted without
   any encryption, exposing it in clear text.
*/
use std::thread;

pub struct Config {
    pub secret: String,
}

impl Config {
    pub fn new() -> Self {
        // In a real application, the secret might be read from a config file or env.
        Self {
            secret: "supersecret".to_string(),
        }
    }
}

fn transmit(cfg: &Config) -> String {
    let data: String;
    unsafe {
        // SAFETY: We are directly creating a slice from the secret's raw pointer.
        // This bypasses normal checks and can be misused to leak sensitive data.
        let ptr = cfg.secret.as_ptr();
        let len = cfg.secret.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        data = String::from_utf8_unchecked(slice.to_vec());
    }
    data  // Returns the cleartext secret.
}

pub fn process_output() -> String {
    let cfg = Config::new();
    // Spawn a separate thread to simulate concurrent handling.
    let handle = thread::spawn(move || {
        transmit(&cfg)
    });
    handle.join().unwrap()
}

fn main() {
    // The main function mimics an application transmitting sensitive data.
    let output = process_output();
    println!("Transmitted data: {}", output);
}