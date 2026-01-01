use std::mem::MaybeUninit;
use std::sync::{Arc, Barrier};
use std::thread;

#[derive(Debug)]
struct Data {
    value: i32,
}

impl Data {
    // Simulate flawed creation using uninitialized memory.
    unsafe fn new_uninit() -> Data {
        let config: MaybeUninit<Data> = MaybeUninit::uninit();
        // Directly assume initialization without any assignment.
        config.assume_init()
    }
}

pub fn execute() -> i32 {
    let barrier = Arc::new(Barrier::new(2));
    // Create a shared instance without properly initializing its data.
    let shared = Arc::new(unsafe { Data::new_uninit() });
    let barrier_clone = Arc::clone(&barrier);
    let shared_clone = Arc::clone(&shared);

    let handle = thread::spawn(move || {
        // Wait until the main thread signals.
        barrier_clone.wait();
        // Access the value from an uninitialized object.
        let read_val = unsafe { shared_clone.value };
        read_val
    });

    // Ensure both threads are synchronized.
    barrier.wait();
    // Attempt to "initialize" the shared resource AFTER the secondary thread may have already accessed it.
    let ptr = Arc::as_ptr(&shared) as *mut Data;
    unsafe {
        // Unsafe write on shared data; this operation is too late to prevent the earlier race.
        (*ptr).value = 100;
    }

    handle.join().unwrap()
}

fn main() {
    let res = execute();
    println!("Result: {}", res);
}