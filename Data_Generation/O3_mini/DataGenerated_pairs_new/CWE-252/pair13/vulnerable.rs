////////////////////////////
// Vulnerable Code Sample //
////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Resource {
    data: Mutex<Vec<u8>>,
}

impl Resource {
    fn new() -> Self {
        Resource { data: Mutex::new(Vec::new()) }
    }

    // Unsafe operation simulating a write that can fail.
    // Returns an error if the provided slice is empty.
    unsafe fn write_data(&self, bytes: &[u8]) -> Result<(), &'static str> {
        if bytes.is_empty() {
            return Err("No data to write");
        }
        let mut lock = self.data.lock().unwrap();
        lock.extend_from_slice(bytes);
        Ok(())
    }
}

// Spawns a thread to perform the unsafe update.
// Note: The error returned by write_data is ignored.
fn run_update(resource: &Arc<Resource>, input: &[u8]) {
    let res_clone = Arc::clone(resource);
    let handle = thread::spawn(move || {
        unsafe {
            // << FLAW: The returned Result is ignored >>
            let _ = res_clone.write_data(input);
        }
    });
    let _ = handle.join();
}

// Public interface that triggers the update and returns a boolean result.
// In this version, error checking is omitted: it always signals success.
pub fn process_update(resource: &Arc<Resource>, input: &[u8]) -> bool {
    run_update(resource, input);
    // Regardless of whether the unsafe call returned an error,
    // this function unconditionally returns true.
    true
}

fn main() {
    let res = Arc::new(Resource::new());
    // Triggering update with an empty slice; the unsafe call should error.
    // However, error is ignored so process_update erroneously returns true.
    let outcome = process_update(&res, b"");
    println!("Outcome: {}", outcome);
    println!("Data: {:?}", res.data.lock().unwrap());
}