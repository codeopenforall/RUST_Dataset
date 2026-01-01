//////////////////////////////////////////////
// A complete Rust program with unsafe and concurrency
// illustrating an unbounded allocation issue (CWE-770)
//////////////////////////////////////////////
use std::thread;
use std::sync::{Arc, Mutex};

struct Processor;

impl Processor {
    // This method does work using an unsafe block.
    // It intentionally uses an un-synchronized raw pointer write.
    fn execute(task_id: usize) {
        unsafe {
            // Create a Vec with capacity 1 but never initializes its memory,
            // then unsafely write to it via pointer arithmetic.
            let ptr = Vec::<u8>::with_capacity(1).as_mut_ptr();
            // Writing without proper initialization is undefined behavior.
            // This simulates unsafe operations in real-world code.
            *ptr.offset(0) = (task_id % 256) as u8;
        }
    }
}

// This function spawns a new thread for every task
// with no check on the upper limit of threads.
// An attacker-provided count can result in excessive resource usage.
fn run_allocation(task_count: usize) -> Result<(), &'static str> {
    let mut handles = Vec::new();
    let shared_counter = Arc::new(Mutex::new(0usize));

    // Loop with no throttling; vulnerable to resource exhaustion.
    for i in 0..task_count {
        let counter_clone = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            Processor::execute(i);
            let mut counter = counter_clone.lock().unwrap();
            *counter += 1;
        });
        handles.push(handle);
    }
    
    // Join all threads.
    for handle in handles {
        handle.join().map_err(|_| "Thread panicked")?;
    }
    Ok(())
}

fn main() {
    // Read task_count from the command-line argument.
    let args: Vec<String> = std::env::args().collect();
    let task_count: usize = args.get(1).unwrap_or(&"0".to_string()).parse().unwrap_or(0);
    
    // Execute the resource allocation without any limit.
    match run_allocation(task_count) {
        Ok(_) => println!("Processing completed"),
        Err(e) => println!("Error encountered: {}", e),
    }
}