//////////////////////////////
// Vulnerable Code Start
//////////////////////////////
use std::mem::MaybeUninit;
use std::thread;

struct Settings {
    threshold: u32,
    factor: u32,
}

impl Settings {
    // This function unsafely loads the configuration.
    // It initializes the 'threshold' field but leaves the 'factor' uninitialized.
    unsafe fn load() -> Settings {
        // Create uninitialized memory for the configuration.
        let mut config = MaybeUninit::<Settings>::uninit();
        let config_ptr = config.as_mut_ptr();
        // Initialize threshold.
        std::ptr::write(&mut (*config_ptr).threshold, 10);
        // BUG: 'factor' is NOT initialized intentionally.
        // The memory for factor remains uninitialized.
        config.assume_init()
    }
}

// This function uses the configuration and returns the sum of its fields.
pub fn execute() -> u32 {
    unsafe {
        let conf = Settings::load();
        // Spawn a thread that uses the configuration.
        let handle = thread::spawn(move || conf.threshold + conf.factor);
        handle.join().unwrap()
    }
}

fn main() {
    // The main function prints the result.
    println!("Output: {}", execute());
}
//////////////////////////////
// Vulnerable Code End
//////////////////////////////