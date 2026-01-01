////////////////////////////////////////////
// Corrected Code for CWE-755
////////////////////////////////////////////
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    secret: i32,
}

impl Config {
    // Securely loads configuration from the environment.
    // Returns an error if the variable is missing, malformed, or zero.
    fn load() -> Result<Self, &'static str> {
        let value = env::var("CONFIG_VAL").map_err(|_| "missing CONFIG_VAL")?;
        let secret = value.parse::<i32>().map_err(|_| "CONFIG_VAL not an integer")?;
        if secret == 0 {
            return Err("CONFIG_VAL cannot be zero");
        }
        Ok(Config { secret })
    }
}

// Processes an input number using the configuration parameter.
// Safe division is used as the configuration is validated.
fn compute(input: i32, config: &Config) -> i32 {
    // Since secret is validated to be non-zero, normal division is safe.
    input / config.secret
}

// Core application function that spawns a thread and computes a result.
// Returns a Result to allow proper error handling.
fn execute() -> Result<(), &'static str> {
    let config = Config::load()?;
    let shared = Arc::new(Mutex::new(config));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let conf = shared_clone.lock().unwrap();
        compute(100, &conf)
    });
    let _res = handle.join().map_err(|_| "Thread panicked")?;
    println!("Execution completed without panic.");
    Ok(())
}

fn main() {
    if let Err(e) = execute() {
        eprintln!("Error encountered: {}", e);
        std::process::exit(1);
    }
}