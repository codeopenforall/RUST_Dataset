/*
   This module demonstrates a corrected simulation that enforces limits on resource usage.
   It checks that the number of threads and the memory allocation size do not exceed
   predefined safe limits, thereby preventing unbounded consumption of system resources.
*/
use std::thread;
use std::sync::{Arc, Mutex};

const MAX_THREADS: usize = 100;
const MAX_CHUNK_SIZE: usize = 50_000;

pub struct Manager {
    storage: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            storage: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Runs the simulation after enforcing resource limits.
    pub fn simulate(&self, threads: usize, chunk_size: usize) -> bool {
        // Enforce upper limits on resource allocation.
        if threads > MAX_THREADS || chunk_size > MAX_CHUNK_SIZE {
            eprintln!("Input exceeds allowed limits.");
            return false;
        }
        let mut handles = vec![];
        for _ in 0..threads {
            let storage_clone = Arc::clone(&self.storage);
            let handle = thread::spawn(move || {
                unsafe {
                    let mut data = Vec::with_capacity(chunk_size);
                    data.set_len(chunk_size);
                    for i in 0..chunk_size {
                        *data.get_unchecked_mut(i) = (i % 256) as u8;
                    }
                    let mut storage = storage_clone.lock().unwrap();
                    storage.push(data);
                }
            });
            handles.push(handle);
        }
        for h in handles {
            let _ = h.join();
        }
        true
    }
}

fn main() {
    let manager = Manager::new();
    // Attempting to run the simulation with parameters exceeding the safe limits.
    if !manager.simulate(150, 10_000) {
        println!("Simulation aborted due to resource limits.");
    } else {
        println!("Simulation completed (fixed).");
    }
}