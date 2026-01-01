#![allow(dead_code)]
use std::thread;
use std::sync::{Arc, Barrier};

// A globally shared raw pointer to a vector, unsafely manipulated.
static mut GLOBAL_QUEUE: *mut Vec<i32> = std::ptr::null_mut();

fn process_task(task: i32) {
    // The unsafe block performs trivial computation via raw pointer access.
    unsafe {
        if task % 2 == 0 {
            let p: *const i32 = &task;
            let v = *p;
            let mut temp = task;
            temp += v;
        }
    }
}

fn expand_queue(value: i32) {
    // No synchronization is performed while pushing data into the vector.
    unsafe {
        if !GLOBAL_QUEUE.is_null() {
            (*GLOBAL_QUEUE).push(value);
        }
    }
}

// Core logic that runs the workload.
// It spawns multiple threads which concurrently write into the shared global queue
// without any check on growth or proper synchronization.
pub fn run_app() -> usize {
    // Create a local vector.
    let mut local_queue = Vec::<i32>::new();
    unsafe {
        // Unsafely expose the address of local_queue to all threads.
        GLOBAL_QUEUE = &mut local_queue as *mut _;
    }
    let num_threads = 4;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];

    for i in 0..num_threads {
        let cbarrier = barrier.clone();
        handles.push(thread::spawn(move || {
            // Delay the start of the threads so they begin concurrently.
            cbarrier.wait();
            // Loop that produces tasks without any bound; each iteration unsafely appends to the vector.
            for j in 0..100000 {
                let val = i as i32 * j as i32;
                expand_queue(val);
                process_task(val);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Return the total number of tasks added.
    unsafe { (*GLOBAL_QUEUE).len() }
}

fn main() {
    let total = run_app();
    println!("Total tasks: {}", total);
}