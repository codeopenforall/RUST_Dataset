/*
This example demonstrates a resource exhaustion vulnerability. The program creates an unbounded number of threads based solely on user input without any throttling or limit checking. In addition, it employs unsafe blocks to manipulate a shared global resource, performing unchecked raw pointer writes. This can lead to memory corruption or denial‐of‐service if a malicious user requests an extremely large number of threads.
*/
use std::env;
use std::thread;
use std::sync::{Arc, Mutex};

static mut GLOBAL_RESOURCE: Option<Arc<Mutex<Vec<u8>>>> = None;

fn spawn_workers(count: usize) {
    // The resource is allocated without initial capacity.
    let resource = Arc::new(Mutex::new(Vec::new()));
    unsafe {
        GLOBAL_RESOURCE = Some(resource.clone());
    }
    // No cap on the number of threads; may lead to exhaustion.
    for _ in 0..count {
        let res = resource.clone();
        thread::spawn(move || {
            // Unsafe memory manipulation on the shared vector.
            unsafe {
                let mut guard = res.lock().unwrap();
                // Fetch raw pointer - vector might be empty!
                let ptr = guard.as_mut_ptr();
                if !ptr.is_null() {
                    // Write to the first byte disregarding vector length.
                    *ptr = 42;
                }
            }
        });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // User-controlled thread count, defaulting to an excessively high value.
    let count = args.get(1)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(1_000_000);
    spawn_workers(count);
    println!("Spawned {} workers", count);
}