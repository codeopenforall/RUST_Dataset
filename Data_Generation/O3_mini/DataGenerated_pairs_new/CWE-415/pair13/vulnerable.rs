#![allow(unused)]
use std::sync::atomic::{AtomicBool, Ordering};

struct Data {
    value: i32,
}

impl Data {
    fn new(v: i32) -> Self {
        Data { value: v }
    }
}

// Global flag to simulate double free detection.
static TRIGGERED: AtomicBool = AtomicBool::new(false);

pub fn run() -> i32 {
    // Allocate a resource on the heap.
    let resource = Box::new(Data::new(10));
    // Obtain the raw pointer; ownership is transferred.
    let raw_ptr = Box::into_raw(resource);
    unsafe {
        // Reconstruct the first Box from the raw pointer.
        let box_a = Box::from_raw(raw_ptr);
        // Simulate a vulnerability by erroneously reconstructing a second Box from the same pointer.
        // This simulates a double free: when both boxes go out of scope their destructors will try to free the same memory.
        if TRIGGERED.swap(true, Ordering::SeqCst) {
            // In a real scenario undefined behavior would ensue.
            // Here we force a panic to make detection easier in the test oracle.
            panic!("double free detected");
        }
        let box_b = Box::from_raw(raw_ptr);
        // Use both instances to compute a result.
        let sum = box_a.value + box_b.value;
        sum
    }
}

fn main() {
    // Running vulnerable code.
    println!("Result: {}", run());
}