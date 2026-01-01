/* 
   This code demonstrates a configuration loader that uses an unsafe block and MaybeUninit 
   incorrectly. One of the fields in the configuration struct remains uninitialized, so later 
   concurrent reads via an Arc lead to use of an uninitialized resource.
*/
use std::mem::MaybeUninit;
use std::ptr;
use std::sync::Arc;
use std::thread;

struct Config {
    threshold: u32,
    desc: String, // Uninitialized field!
}

fn load_config() -> Config {
    // Allocate the memory for Config without initializing all fields.
    let mut cfg = MaybeUninit::<Config>::uninit();
    unsafe {
        let cfg_ptr = cfg.as_mut_ptr();
        // Initialize the threshold correctly.
        ptr::write(&mut (*cfg_ptr).threshold, 10);
        // NOTE: The field 'desc' is intentionally left uninitialized!
        // Now we incorrectly assume full initialization.
        cfg.assume_init()
    }
}

fn main() {
    // Load the configuration unsafely.
    let conf = Arc::new(load_config());

    // Spawn a thread that concurrently inspects the configuration.
    let conf_clone = Arc::clone(&conf);
    let handle = thread::spawn(move || {
        // This check expects a valid string, but 'desc' is uninitialized.
        if conf_clone.desc.is_empty() {
            panic!("Configuration description is uninitialized or empty!");
        }
    });

    // Wait for the thread to complete.
    handle.join().expect("Thread panicked");

    // Use the configuration.
    println!("Threshold: {} - Desc: {}", conf.threshold, conf.desc);
}