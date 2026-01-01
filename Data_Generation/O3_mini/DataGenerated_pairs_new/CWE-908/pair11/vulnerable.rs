//////////////////////////////////////////////////////////////
// Vulnerable sample code demonstrating unsafe use of uninitialized
// memory via MaybeUninit in a concurrent context.
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

// UNSAFE: This function creates storage for Settings but never writes a value
// into it before assuming it is initialized.
unsafe fn setup() -> Settings {
    let slot: MaybeUninit<Settings> = MaybeUninit::uninit();
    // POTENTIAL FLAW (CWE-908): Using uninitialized memory!
    slot.assume_init()
}

// Public function providing an API for running the application logic.
pub fn run_app() -> u32 {
    // In a multithreaded context, the configuration is shared.
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