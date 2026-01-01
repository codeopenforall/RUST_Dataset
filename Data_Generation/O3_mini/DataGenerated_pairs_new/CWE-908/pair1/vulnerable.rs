//////////////////////////////////////////////////////////////////////////
// Vulnerable Code - CWE-908: Use of Uninitialized Resource
//////////////////////////////////////////////////////////////////////////
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::thread;

struct Settings {
    timeout: u64,
    retries: u32,
}

impl Settings {
    // This method unsafely initializes only a subset of the fields.
    unsafe fn load() -> Self {
        let mut config = MaybeUninit::<Settings>::uninit();
        let ptr = config.as_mut_ptr();
        // Initialize only the 'timeout' field.
        (*ptr).timeout = 30;
        // The 'retries' field is left uninitialized.
        config.assume_init()
    }
}

fn main() {
    // Simulate concurrent access to the configuration.
    let shared = Arc::new(unsafe { Settings::load() });
    let cloned = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        // Accessing an uninitialized field may lead to undefined behavior.
        if cloned.retries > 0 {
            println!("Retries is set to {}", cloned.retries);
        } else {
            println!("Retries is zero or uninitialized, value: {}", cloned.retries);
        }
    });
    handle.join().unwrap();
    println!("Timeout is: {}", shared.timeout);
}