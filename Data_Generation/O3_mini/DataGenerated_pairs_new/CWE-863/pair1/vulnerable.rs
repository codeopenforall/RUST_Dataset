////////////////////////////////////////////////////////////////
// Vulnerable Code
////////////////////////////////////////////////////////////////
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

static mut LIMIT: usize = 1000;

trait Worker {
    fn execute(&self);
}

struct Task {
    data: usize,
}

impl Worker for Task {
    fn execute(&self) {
        // Simulate some work
        thread::sleep(Duration::from_millis(1));
    }
}

struct ResourceManager {
    tasks: Arc<Mutex<Vec<thread::JoinHandle<()>>>>,
}

impl ResourceManager {
    fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    // Function that allocates tasks without enforcing the resource cap.
    // CWE-863: Missing capacity enforcement can allow allocation of too many threads/tasks.
    fn allocate(&self, count: usize) -> Result<(), String> {
        // Unsafe read of the global limit (but not used for validation).
        let _limit = unsafe { LIMIT };
        // No check is performed: any count, even exceeding the intended limit,
        // results in spawning and collecting new threads.
        for i in 0..count {
            let task = Task { data: i };
            // Unsafe block for thread spawning (simulating complex unsafe-concurrency usage)
            let handle = {
                // Using an inner unsafe block to mimic complex patterns.
                unsafe {
                    thread::spawn(move || {
                        task.execute();
                    })
                }
            };
            self.tasks.lock().unwrap().push(handle);
        }
        Ok(())
    }
    
    fn join_all(&self) {
        let mut lock = self.tasks.lock().unwrap();
        while let Some(handle) = lock.pop() {
            let _ = handle.join();
        }
    }
}

fn main() {
    let manager = ResourceManager::new();
    // Demonstration allocation with a small count.
    // Vulnerable to misuse if caller does not restrict the count input.
    manager.allocate(10).expect("Allocation failed unexpectedly");
    manager.join_all();
    println!("Execution completed in vulnerable build.");
}