////////////////////////////////////////////////////////////
// Vulnerable Code Sample
////////////////////////////////////////////////////////////
#![allow(dead_code)]
use std::mem::MaybeUninit;

#[repr(C)]
pub struct Config {
    pub name: String,
    pub count: u32,
}

impl Config {
    // This function misuses MaybeUninit: it only initializes the "count" field
    // leaving "name" uninitialized. Later usage of "name" (e.g. in valid())
    // leads to use of uninitialized memory.
    pub unsafe fn build_partial(count: u32) -> Self {
        let mut storage = MaybeUninit::<Config>::uninit();
        let ptr = storage.as_mut_ptr();
        // Intentionally initialize only the count field.
        // Calculate pointer offset for the count field.
        // (Since Config is #[repr(C)], the fields follow declaration order.)
        // Do NOT initialize the name field.
        let count_ptr = &mut (*ptr).count as *mut u32;
        count_ptr.write(count);
        // Note: "name" field remains uninitialized.
        storage.assume_init()
    }

    // This method uses the uninitialized "name" field.
    pub fn valid(&self) -> bool {
        // Attempt to use the String value.
        // This call on an uninitialized String invokes undefined behavior.
        !self.name.is_empty() && self.count > 0
    }
}

// Public function used as the contract for testing.
pub fn compute() -> bool {
    // UNSAFE: use the function that builds a partially initialized structure.
    unsafe {
        let cfg = Config::build_partial(42);
        cfg.valid()
    }
}

fn main() {
    // In a real application this might lead to unpredictable behavior or a crash.
    let result = std::panic::catch_unwind(|| {
        // This call is expected to trigger undefined behavior.
        compute()
    });
    match result {
        Ok(validity) => println!("Configuration valid: {}", validity),
        Err(_) => println!("Runtime error occurred during configuration processing."),
    }
}