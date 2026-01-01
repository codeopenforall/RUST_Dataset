////////////////////////////////////////////////////////////////////////////////
// This corrected version properly initializes all fields of the resource.
////////////////////////////////////////////////////////////////////////////////
use std::mem::MaybeUninit;
use std::sync::{Arc, Barrier};
use std::thread;

struct Config {
    threshold: i32,
    description: String,
}

impl Config {
    fn new(threshold: i32, description: String) -> Self {
        Config { threshold, description }
    }
}

fn load_resource(trigger: i32) -> Config {
    let mut data: MaybeUninit<Config> = MaybeUninit::uninit();
    unsafe {
        let ptr = data.as_mut_ptr();
        // Fully initialize both fields.
        ptr.write(Config::new(trigger, "Properly initialized".to_string()));
        data.assume_init()
    }
}

fn main() {
    let barrier = Arc::new(Barrier::new(2));
    let resource = load_resource(42);
    let shared = Arc::new(resource);
    let barrier_clone = barrier.clone();
    let shared_copy = shared.clone();

    let handle = thread::spawn(move || {
        barrier_clone.wait();
        // Safely access the fully initialized 'description' field.
        let len = shared_copy.description.len();
        len
    });

    barrier.wait();
    let thread_len = handle.join().expect("Thread panicked");
    println!(
        "Threshold: {}, Description length: {}",
        shared.threshold, thread_len
    );
}