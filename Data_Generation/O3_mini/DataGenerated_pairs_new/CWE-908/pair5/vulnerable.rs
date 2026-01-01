use std::mem::MaybeUninit;
use std::thread;

struct Resources {
    title: String,
    amount: u32,
}

impl Resources {
    // This unsafe constructor tries to initialize only one field of the struct.
    // It deliberately leaves "amount" uninitialized.
    unsafe fn new_partial() -> Self {
        let mut uninit = MaybeUninit::<Resources>::uninit();
        let ptr = uninit.as_mut_ptr();
        // Only the "title" field is properly set.
        (*ptr).title = String::from("configuration");
        // "amount" remains uninitialized intentionally.
        uninit.assume_init()
    }
}

fn run_app() -> u32 {
    // An instance of Resources is built using the unsafe, partial initializer.
    // This will result in "amount" holding an indeterminate value.
    let config = unsafe { Resources::new_partial() };
    // Spawn a thread to simulate concurrent usage of the uninitialized data.
    let handle = thread::spawn(move || {
        // The uninitialized "amount" field is read here.
        config.amount
    });
    handle.join().unwrap()
}

fn main() {
    let result = run_app();
    println!("Result: {}", result);
}