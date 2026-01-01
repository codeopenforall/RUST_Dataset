//////////////////////////////////////////////////////////////
// Corrected sample code that properly initializes the resource
// before use. This eliminates the undefined behavior associated
// with reading uninitialized memory.
//////////////////////////////////////////////////////////////
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::thread;

struct Settings {
    flag: u32,
}

impl Settings {
    fn new(val: u32) -> Self {
        Settings { flag: val }
    }

    fn flag(&self) -> u32 {
        self.flag
    }
}

// Safe function that ensures the Settings object is fully initialized before use.
unsafe fn setup() -> Settings {
    let mut slot: MaybeUninit<Settings> = MaybeUninit::uninit();
    // PROPER INITIALIZATION: Write a valid Settings instance into memory.
    slot.as_mut_ptr().write(Settings::new(42));
    slot.assume_init()
}

// Public API for running the application logic.
pub fn run_app() -> u32 {
    let shared = Arc::new(unsafe { setup() });
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        shared_clone.flag()
    });
    handle.join().unwrap()
}

fn main() {
    let result = run_app();
    println!("Result flag: {}", result);
}