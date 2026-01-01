///////////////////////////////////////////////////////////////
// Vulnerable Code Example
///////////////////////////////////////////////////////////////
#![allow(dead_code)]
use std::thread;
use std::sync::Arc;
use std::ptr;

struct Handler {
    threshold: u32,
}

impl Handler {
    // In this implementation, failure during configuration loading is silently swallowed.
    // Instead of propagating the error, unwrap_or_default is used,
    // ultimately setting the threshold to 0 (an insecure default).
    fn new() -> Self {
        // Attempt to load configuration (which will always fail in this simulation)
        let config_content = load_config("config.txt").unwrap_or_default(); // Vulnerability: error swallowed
        // Parse the threshold from the loaded configuration.
        // On failure (e.g. empty string), the default value 0 is used.
        let threshold = config_content.trim().parse::<u32>().unwrap_or_default(); // Vulnerability: parsing error swallowed
        Handler { threshold }
    }
}

unsafe fn get_global() -> &'static Handler {
    &*GLOBAL_HANDLE
}

static mut GLOBAL_HANDLE: *mut Handler = ptr::null_mut();

// A simulated configuration loader which always returns an error.
fn load_config(_path: &str) -> Result<String, &'static str> {
    Err("failed to load configuration")
}

fn main() {
    // Create a new configuration handler; errors are swallowed resulting in an insecure default.
    let instance = Handler::new();
    let boxed = Box::new(instance);
    unsafe {
        GLOBAL_HANDLE = Box::into_raw(boxed);
    }

    // Spawn a thread to simulate concurrent usage of the configuration.
    let handle_arc = Arc::new(());
    let thread_handle = {
        let arc_clone = Arc::clone(&handle_arc);
        thread::spawn(move || {
            let _ = arc_clone; // Simulate additional shared state usage.
            unsafe {
                let conf = get_global();
                // Use the threshold for a critical decision.
                if conf.threshold == 0 {
                    // This insecure default may lead to unintended behavior.
                    println!("Warning: insecure default threshold in use!");
                } else {
                    println!("Threshold set securely to: {}", conf.threshold);
                }
            }
        })
    };

    thread_handle.join().unwrap();

    unsafe {
        // Clean up to prevent memory leak.
        if !GLOBAL_HANDLE.is_null() {
            drop(Box::from_raw(GLOBAL_HANDLE));
            GLOBAL_HANDLE = ptr::null_mut();
        }
    }
}