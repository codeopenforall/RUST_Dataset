use std::mem::MaybeUninit;
use std::thread;

struct Config {
    value: i32,
    message: String,
}

fn initialize() -> Config {
    // Correctly initialize the complete structure.
    let mut config = MaybeUninit::<Config>::uninit();
    unsafe {
        let config_ptr = config.as_mut_ptr();

        // Fully initialize all fields.
        (*config_ptr).value = 42;
        // Properly initialize the 'message' field using a write to the uninitialized memory location.
        std::ptr::write(&mut (*config_ptr).message, String::from("Initialized!"));

        // A concurrent thread that simulates additional operations.
        let handle = thread::spawn(|| {
            // dummy concurrent operation; no interference with the already fully initialized structure.
        });
        handle.join().unwrap();

        // Now that all fields are set, safely assume initialization.
        config.assume_init()
    }
}

fn run_app() -> String {
    let cfg = initialize();
    cfg.message
}

fn main() {
    let output = run_app();
    println!("{}", output);
}