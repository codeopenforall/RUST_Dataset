#![allow(unused_unsafe)]
use std::thread;
use std::sync::{Arc, Mutex};

trait Arithmetic {
    fn compute(&self) -> i32;
}

struct Container {
    data: i32,
}

impl Arithmetic for Container {
    fn compute(&self) -> i32 {
        // Simple computation: add one to the internal data.
        self.data + 1
    }
}

fn process() -> i32 {
    // Allocate memory for the resource.
    let resource = Box::new(Container { data: 99 });
    // Convert the Box into a raw pointer.
    let raw = Box::into_raw(resource);

    // Incorrectly free the memory by reconstructing and dropping the Box.
    // This leaves `raw` pointing to freed memory.
    unsafe {
        let _ = Box::from_raw(raw); // Vulnerability: memory is freed here.
    }

    // Use-after-free: Dereferencing the dangling pointer.
    let result = unsafe { (*raw).compute() }; // Vulnerability: using memory after free.
    result
}

fn main() {
    // Call the process function that contains unsafe memory usage.
    let val = process();
    println!("Result: {}", val);

    // Additionally, spawn a thread that demonstrates similar misuse.
    let _handle = thread::spawn(|| {
        let boxed = Box::new(Container { data: 50 });
        let raw_ptr = Box::into_raw(boxed);
        unsafe {
            let _ = Box::from_raw(raw_ptr); // Freeing the memory.
        }
        // Unsafe use of the memory after it has been released.
        let wrong_use = unsafe { (*raw_ptr).compute() };
        println!("Thread result: {}", wrong_use);
    });
    // Give the thread time to run.
    thread::sleep(std::time::Duration::from_millis(50));
}