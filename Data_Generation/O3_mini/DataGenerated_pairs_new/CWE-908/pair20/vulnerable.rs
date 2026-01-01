use std::mem::MaybeUninit;
use std::thread;

struct Config {
    value: i32,
    message: String,
}

fn initialize() -> Config {
    // UNSAFE: Partially initializing a Config instance.
    unsafe {
        // Create uninitialized memory for Config
        let mut config: MaybeUninit<Config> = MaybeUninit::uninit();            // Line 7
        let config_ptr = config.as_mut_ptr();

        // Initialize only the integer field
        (*config_ptr).value = 42;                                                 // Line 10
        // BUG: The string field 'message' is left uninitialized

        // Simulate some concurrency that might exacerbate timing issues
        let handle = thread::spawn(|| {
            // dummy concurrent operation: no synchronization with the initialization
        });
        handle.join().unwrap();

        // Assume full initialization and return the data (BUG: message is uninitialized)
        config.assume_init()                                                       // Line 13
    }
}

fn run_app() -> String {
    let cfg = initialize();
    // Return the message field; this could cause undefined behavior
    cfg.message
}

fn main() {
    let output = run_app();
    println!("{}", output);
}