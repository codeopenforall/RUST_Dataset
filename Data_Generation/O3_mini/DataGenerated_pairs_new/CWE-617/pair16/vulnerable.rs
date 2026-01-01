//////////////////////////////////////////////////////////////
// Vulnerable Code: CWE-617 - Reachable Assertion via Attacker-Controlled Input
//////////////////////////////////////////////////////////////
use std::env;
use std::ptr;
use std::thread;
use std::time::Duration;

struct System {
    threshold: usize,
}

impl System {
    fn new(value: usize) -> Self {
        System { threshold: value }
    }

    // The invariant is that threshold must be less than 100.
    // An attacker-controlled value that violates this invariant triggers an assertion panic.
    fn process(&self) {
        // Vulnerability: The threshold is not validated and the assertion is directly reached.
        assert!(self.threshold < 100, "Threshold too high, possible DoS");
    }
}

// Global static pointer used unsafely without synchronization.
static mut GLOBAL_SYSTEM: *mut System = ptr::null_mut();

// Loads the configuration from an attacker-supplied value without proper validation.
fn load_configuration(value: usize) {
    unsafe {
        GLOBAL_SYSTEM = Box::into_raw(Box::new(System::new(value)));
    }
}

// Retrieves a reference to the global configuration unsafely.
fn get_configuration() -> &'static System {
    unsafe {
        if GLOBAL_SYSTEM.is_null() {
            panic!("Configuration not loaded");
        }
        &*GLOBAL_SYSTEM
    }
}

// The core function that simulates concurrent processing.
// It reads the input from args, loads configuration unsafely, and spawns a thread.
fn run(args: Vec<String>) {
    let input = args.get(1).expect("Missing argument");
    let value: usize = input.parse().expect("Invalid number provided");
    load_configuration(value);

    // Spawn a thread to simulate concurrent access.
    let handle = thread::spawn(|| {
        unsafe {
            thread::sleep(Duration::from_millis(50));
            let config = get_configuration();
            config.process();
        }
    });

    unsafe {
        let config = get_configuration();
        config.process();
    }
    handle.join().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    run(args);
}