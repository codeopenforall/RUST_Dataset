/* 
   This module demonstrates a simulation that spawns a configurable number of threads.
   It allocates memory unsafely without checking if the number of threads or the allocation
   size is within a safe limit. Such unbounded allocation can exhaust system resources.
*/
use std::thread;
use std::sync::{Arc, Mutex};

pub struct Manager {
    storage: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            storage: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Runs the simulation by spawning threads without resource limits.
    pub fn simulate(&self, threads: usize, chunk_size: usize) -> bool {
        let mut handles = vec![];
        for _ in 0..threads {
            let storage_clone = Arc::clone(&self.storage);
            // Spawn a new thread that performs unsafe operations.
            let handle = thread::spawn(move || {
                unsafe {
                    // Allocate a vector with the given capacity and force its length.
                    let mut data = Vec::with_capacity(chunk_size);
                    data.set_len(chunk_size);
                    // Populate the vector unsafely.
                    for i in 0..chunk_size {
                        *data.get_unchecked_mut(i) = (i % 256) as u8;
                    }
                    // Unrestricted insertion into shared storage.
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
    // No limits are enforced on the number of threads or the chunk size.
    manager.simulate(150, 10_000);
    println!("Simulation completed (vulnerable).");
}