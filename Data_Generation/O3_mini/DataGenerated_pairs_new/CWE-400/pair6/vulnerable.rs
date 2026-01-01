//////////////////////////////////////////
// Vulnerability Simulation Code Example
//
// Description:
// This code simulates an uncontrolled resource consumption vulnerability
// (CWE-400) in a concurrent setting. Multiple producer threads push messages
// to an unbounded channel with no backpressure. An atomic counter is used to
// simulate load tracking, and when it exceeds a predefined limit, the producers
// panic – simulating a resource exhaustion scenario. An unsafe function is used
// to process messages through a raw pointer read, emulating unsafe Rust usage.
// Note that the panic due to exceeding the resource limit is our “vulnerability.”
//////////////////////////////////////////

use std::sync::{mpsc, Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::time::Duration;
use std::ptr;

const LIMIT: usize = 100;

fn unsafe_transform(x: u32) -> u32 {
    // Unsafe raw pointer read to mimic low-level operations.
    unsafe {
        let ptr = &x as *const u32;
        ptr::read(ptr)
    }
}

pub fn run_simulation(iterations: usize) -> Result<usize, String> {
    let (tx, rx) = mpsc::channel(); // Unbounded channel without backpressure.
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    // Spawn 4 concurrent producer threads.
    for thread_index in 0..4 {
        let thread_tx = tx.clone();
        let thread_counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for j in 0..iterations {
                // Use an unsafe function to simulate low-level processing.
                let msg = unsafe_transform(j as u32);
                // Increase the load count.
                let load = thread_counter.fetch_add(1, Ordering::Relaxed) + 1;
                // No backpressure here: if load exceeds LIMIT, panic.
                if load > LIMIT {
                    // Vulnerability triggered: uncontrolled accumulation causes DoS.
                    panic!("Resource limit exceeded in producer thread {}", thread_index);
                }
                thread_tx.send(msg).unwrap();
                // Immediately continue loop without any delay.
            }
        }));
    }

    drop(tx); // Close the sender so that the consumer can eventually exit.

    let mut processed = 0;
    // Consumer processes messages slowly to exacerbate accumulation.
    while let Ok(msg) = rx.recv() {
         let _ = unsafe_transform(msg);
         processed += 1;
         // Slow down processing to trigger uncontrolled growth.
         thread::sleep(Duration::from_millis(1));
         counter.fetch_sub(1, Ordering::Relaxed);
    }

    for h in handles {
         let _ = h.join();
    }
    Ok(processed)
}

fn main() {
    match run_simulation(1000) {
         Ok(n) => println!("Processed {} messages", n),
         Err(e) => eprintln!("Error: {}", e),
    }
}