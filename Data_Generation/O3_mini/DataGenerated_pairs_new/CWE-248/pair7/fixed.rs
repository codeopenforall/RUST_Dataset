/////////////////// Fixed Version ////////////////////////
use std::thread;
use std::panic;

struct Worker;

impl Worker {
    // Process the input value and return an Option with the computed result.
    // The fixed method uses catch_unwind to intercept panics, turning them into an error code.
    fn process(&self, input: i32) -> Option<i32> {
        let handle = thread::spawn(move || {
            // Safely call the unsafe operation by catching unwinding.
            // If a panic occurs, return -1 to indicate an error.
            panic::catch_unwind(|| unsafe { Self::unsafe_op(input) })
                .unwrap_or(-1)
        });
        Some(handle.join().unwrap())
    }

    // Unsafe operation that is now paired with proper memory management.
    // It converts the boxed integer to a raw pointer, checks the value, and then safely reclaims memory.
    // Instead of propagating a panic on a zero value, it returns an error code (-1).
    unsafe fn unsafe_op(input: i32) -> i32 {
        let boxed = Box::new(input);
        let raw_ptr = Box::into_raw(boxed);
        let result;
        if *raw_ptr == 0 {
            result = -1;
        } else {
            result = *raw_ptr;
        }
        // Reclaim the memory to prevent a leak.
        drop(Box::from_raw(raw_ptr));
        result
    }
}

fn main() {
    let worker = Worker;
    let res = worker.process(0);
    // Check the returned result: a value of -1 signals that an error was handled gracefully.
    match res {
        Some(val) if val == -1 => println!("Handled error gracefully"),
        Some(val) => println!("Result: {}", val),
        None => println!("No result obtained"),
    }
}