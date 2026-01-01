//////////////// Vulnerable Code Begin
use std::mem::MaybeUninit;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct Config {
    pub value: u32,
    pub valid: bool,
}

pub fn load_config() -> Config {
    // Unsafe initialization using MaybeUninit.
    // Only one field is set while the other remains uninitialized.
    unsafe {
        let mut config: MaybeUninit<Config> = MaybeUninit::uninit();
        // Set the numerical value only.
        (*config.as_mut_ptr()).value = 42;
        // The 'valid' field is left uninitialized, which can lead to unpredictable behavior.
        // Vulnerability: Use of uninitialized data.
        config.assume_init()
    }
}

pub fn run_threads() -> Vec<u32> {
    let config = Arc::new(Mutex::new(load_config()));
    let mut results = Vec::new();
    let mut handles = Vec::new();

    for _ in 0..4 {
        let config_clone = Arc::clone(&config);
        let h = thread::spawn(move || {
            let cfg = config_clone.lock().unwrap();
            // The logic branches based on the uninitialized 'valid' field.
            if cfg.valid {
                cfg.value + 1
            } else {
                cfg.value - 1
            }
        });
        handles.push(h);
    }

    for h in handles {
        results.push(h.join().unwrap());
    }
    results
}

fn main() {
    let results = run_threads();
    for r in results {
        println!("result: {}", r);
    }
}
//////////////// Vulnerable Code End