/////////////////// Vulnerable Version ////////////////////////
use std::thread;

struct Worker;

impl Worker {
    // Process input value and return an Option with the computed result.
    // For valid inputs it returns Some(result), but for input == 0 the inner thread panics.
    fn process(&self, input: i32) -> Option<i32> {
        let handle = thread::spawn(move || {
            // Unsafe block invocation simulating an unchecked FFI-like call.
            unsafe { Self::unsafe_op(input) }
        });
        // The unwrap here will propagate any panic from the spawned thread.
        Some(handle.join().unwrap())
    }

    // Unsafe operation that converts a boxed integer into a raw pointer,
    // then dereferences it. It panics on a boundary value of zero,
    // simulating an abnormal termination which is not caught.
    unsafe fn unsafe_op(input: i32) -> i32 {
        // Allocate on the heap and obtain a raw pointer.
        let raw_ptr = Box::into_raw(Box::new(input));
        if *raw_ptr == 0 {
            // Trigger an abnormal termination if input is zero.
            panic!("Abnormal termination: input zero");
        }
        let result = *raw_ptr;
        // Memory leak: the allocated memory is never reclaimed.
        result
    }
}

fn main() {
    let worker = Worker;
    // Boundary input zero triggers the unsafe operation to panic.
    let res = worker.process(0);
    println!("Result: {:?}", res);
}