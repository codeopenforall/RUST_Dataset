////////////////////////////////////////////
// Vulnerable Code for CWE-755
////////////////////////////////////////////
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    secret: i32,
}

impl Config {
    // Loads configuration from environment.
    // BUG: Errors are silently swallowed with unwrap_or_default,
    // leading to a default value (zero) when CONFIG_VAL is missing or malformed.
    fn load() -> Self {
        let value = env::var("CONFIG_VAL").unwrap_or_default(); // [Line 9]
        let secret = value.parse::<i32>().unwrap_or_default();   // [Line 10]
        Config { secret }
    }
}

// Processes an input number using the configuration parameter.
// BUG: Uses an unsafe block to read the secret and performs division without checking for zero.
fn compute(input: i32, config: &Config) -> i32 {
    unsafe {
        // Unsafe pointer read of the secret value.
        let ptr = &config.secret as *const i32;
        let secret_val = *ptr;
        // Vulnerability: if secret_val is zero then division will panic.
        input / secret_val   // [Line 19]
    }
}

// Core application function that spawns a thread and computes a result.
fn execute() {
    let config = Config::load();
    let shared = Arc::new(Mutex::new(config));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let conf = shared_clone.lock().unwrap();
        compute(100, &conf)
    });
    let res = handle.join().unwrap();
    println!("Result: {}", res);
}

fn main() {
    execute();
}