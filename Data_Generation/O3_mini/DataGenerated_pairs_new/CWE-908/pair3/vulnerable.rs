//////////////////////////////////////////
// Vulnerable Code for uninitialized resource
//////////////////////////////////////////
use std::mem::MaybeUninit;
use std::thread;

struct Configuration {
    value: i32,
    active: bool,
}

impl Configuration {
    // This function constructs a new instance unsafely.
    // It intentionally initializes only the 'value' field and
    // leaves the 'active' flag uninitialized.
    fn create_instance() -> Self {
        unsafe {
            let mut config: MaybeUninit<Configuration> = MaybeUninit::uninit();
            let ptr = config.as_mut_ptr();
            // Initialize only the 'value' field.
            (*ptr).value = 42;
            // BUG: The 'active' field is not set.
            config.assume_init()
        }
    }
    
    // Returns the state of the 'active' flag.
    fn is_active(&self) -> bool {
        self.active
    }
    
    // A dummy function mimicking some processing.
    fn compute(&self) -> i32 {
        if self.is_active() {
            self.value * 2
        } else {
            self.value
        }
    }
}

fn main() {
    // Create configuration instance unsafely
    let config = Configuration::create_instance();
    
    // Spawn a thread to simulate concurrent access.
    let handle = thread::spawn(move || {
        // This will exhibit undefined behavior if 'active' is read.
        config.compute()
    });
    
    let result = handle.join().unwrap();
    println!("Computed result: {}", result);
}