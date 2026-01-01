///////////////////////////
// Vulnerable Version
///////////////////////////
use std::thread;

struct ResourceManager;

impl ResourceManager {
    // This function allocates a Vec<u8> of the requested size without any cap.
    // It returns a Result to match the public interface, but always succeeds.
    pub fn create_resources(&self, count: usize) -> Result<Vec<u8>, String> {
        // Vulnerable allocation: no upper limit check on count.
        unsafe {
            // Using an unsafe block to mimic low-level allocation routines.
            let mut buf: Vec<u8> = Vec::with_capacity(count);
            for i in 0..count {
                buf.push((i % 256) as u8);
            }
            Ok(buf)
        }
    }

    // This function spawns a number of threads without any throttling.
    pub fn spawn_workers(&self, count: usize) {
        let mut handles = Vec::new();
        for i in 0..count {
            // Each thread is spawned without checking or limiting the total thread count.
            let handle = thread::spawn(move || {
                unsafe {
                    // Simulate unsafe pointer operation.
                    let ptr: *const usize = &i;
                    let _ = *ptr;
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            let _ = handle.join();
        }
    }
}

fn main() {
    let manager = ResourceManager;
    // This call uses an arbitrarily large count.
    // In a real-world attack, an external input could trigger runaway resource consumption.
    let resources = manager.create_resources(1_000_000).unwrap();
    println!("Allocated {} bytes", resources.len());
    // Spawning a large number of worker threads without throttling.
    manager.spawn_workers(1000);
}