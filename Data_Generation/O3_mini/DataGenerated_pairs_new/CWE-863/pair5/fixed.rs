/*
This corrected version mitigates the resource exhaustion vulnerability and unsafe memory access by strictly capping the number of threads and ensuring that the shared resource is correctly allocated with a valid element. The unsafe operations have been replaced with safe indexing into a properly sized vector.
*/
use std::env;
use std::thread;
use std::sync::{Arc, Mutex};

static mut GLOBAL_RESOURCE: Option<Arc<Mutex<Vec<u8>>>> = None;
const MAX_WORKERS: usize = 100; // Limit maximum number of worker threads

fn spawn_workers(count: usize) {
    // Enforce a cap on the number of threads.
    let count = if count > MAX_WORKERS { MAX_WORKERS } else { count };
    // Allocate a vector with an initial element so that indexing is safe.
    let resource = Arc::new(Mutex::new(vec![0u8; 1]));
    unsafe {
        GLOBAL_RESOURCE = Some(resource.clone());
    }
    let mut handles = Vec::with_capacity(count);
    for _ in 0..count {
        let res = resource.clone();
        handles.push(thread::spawn(move || {
            let mut guard = res.lock().unwrap();
            guard[0] = 42; // Safe operation on a vector initialized with capacity.
        }));
    }
    // Wait for all threads to complete.
    for handle in handles {
        let _ = handle.join();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Default count is lower to avoid accidental misuse.
    let count = args.get(1)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(50);
    spawn_workers(count);
    println!("Spawned {} workers (capped)", count);
}