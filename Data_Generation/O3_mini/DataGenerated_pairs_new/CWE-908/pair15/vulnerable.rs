/////////////////////// Vulnerable Code ///////////////////////
use std::{mem::MaybeUninit, sync::{Arc, Mutex}, thread};

#[derive(Debug)]
pub struct Config {
    pub timeout: u32,
    pub label: &'static str,
}

impl Config {
    // Unsafe method that creates a new configuration instance
    // but only partially initializes it.
    pub unsafe fn uninit_config() -> Self {
        let mut uninit = MaybeUninit::<Config>::uninit();
        let ptr = uninit.as_mut_ptr();
        // Explicitly initialize the 'timeout' field.
        // Vulnerability: The 'label' field remains uninitialized.
        (*ptr).timeout = 30;
        // Note: Missing initialization for the 'label' field.
        uninit.assume_init()
    }
}

pub fn make_config() -> Config {
    // Using unsafe block to obtain a config structure that is not fully initialized.
    unsafe { Config::uninit_config() }
}

fn main() {
    // Wrap the (potentially uninitialized) configuration in a shared Mutex.
    let config = Arc::new(Mutex::new(make_config()));
    let mut handles = vec![];

    // Spawn multiple threads to simultaneously access the configuration.
    for _ in 0..4 {
        let cfg = Arc::clone(&config);
        let handle = thread::spawn(move || {
            let locked = cfg.lock().unwrap();
            // Unsafe read of the 'label' field may lead to undefined behavior.
            println!("Config timeout: {}, label: {}", locked.timeout, locked.label);
        });
        handles.push(handle);
    }
    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }
}