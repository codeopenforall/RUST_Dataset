//////////////////////////////////////////
// Fixed Code for proper resource initialization
//////////////////////////////////////////
use std::thread;

struct Configuration {
    value: i32,
    active: bool,
}

impl Configuration {
    // Correctly initializes both fields.
    fn create_instance() -> Self {
        Configuration {
            value: 42,
            active: true, // Properly initialize the flag.
        }
    }
    
    // Returns the state of the 'active' flag.
    fn is_active(&self) -> bool {
        self.active
    }
    
    // A function that emulates processing using the config.
    fn compute(&self) -> i32 {
        if self.is_active() {
            self.value * 2
        } else {
            self.value
        }
    }
}

fn main() {
    // Create a fully initialized configuration instance.
    let config = Configuration::create_instance();
    
    // Spawn a thread to simulate concurrent usage.
    let handle = thread::spawn(move || {
        config.compute()
    });
    
    let result = handle.join().unwrap();
    println!("Computed result: {}", result);
}