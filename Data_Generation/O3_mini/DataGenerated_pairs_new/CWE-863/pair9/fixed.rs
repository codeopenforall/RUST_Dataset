//////////////// Fixed Version ////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::alloc::{alloc, dealloc, Layout};

const MAX_TASKS: usize = 1000; // Cap on the number of tasks allowed

struct Processor;

impl Processor {
    // Enforces a maximum limit on tasks to prevent resource exhaustion.
    pub fn heavy_work(&self, tasks: usize) -> Result<usize, String> {
        if tasks > MAX_TASKS {
            return Err("Number of tasks exceeds allowed limit".to_string());
        }
        let counter = Arc::new(Mutex::new(0));
        let mut handles = Vec::with_capacity(tasks);
        for i in 0..tasks {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                unsafe {
                    let size = (i % 1024) + 1; // allocate between 1 and 1024 bytes
                    let layout = Layout::from_size_align(size, 8).unwrap();
                    let mem = alloc(layout);
                    if mem.is_null() {
                        panic!("Memory allocation failed");
                    }
                    *(mem as *mut u8) = (i % 256) as u8;
                    dealloc(mem, layout);
                }
                let mut cnt = counter.lock().unwrap();
                *cnt += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().map_err(|_| "Thread panicked".to_string())?;
        }
        let result = *counter.lock().unwrap();
        Ok(result)
    }
}

fn main() {
    let processor = Processor;
    // Fixed: Enforce a limit on the number of tasks to prevent unbounded allocation.
    let tasks = 1500;
    match processor.heavy_work(tasks) {
        Ok(count) => println!("Completed {} tasks", count),
        Err(e) => println!("Error: {}", e),
    }
}