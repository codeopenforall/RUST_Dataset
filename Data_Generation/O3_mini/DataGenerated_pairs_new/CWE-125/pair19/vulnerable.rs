/////////////////////////////////////////////
// CWE-125 Demonstration: Unsafe Buffer Read  //
// This code intentionally reads out-of-bound //
// memory by misusing unchecked pointer arithmetic//
/////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

// A trait defining a computation contract.
trait Compute {
    fn compute(&self) -> u32;
}

// A structure holding a dynamic buffer.
struct DataHolder {
    data: Vec<u32>,
}

impl Compute for DataHolder {
    // This method computes a sum over adjacent pairs.
    // It uses an unsafe block for pointer arithmetic.
    // The loop iterates over the whole buffer, and in the last iteration
    // it reads one element past the end of the vector, causing an out-of-bound read.
    fn compute(&self) -> u32 {
        let len = self.data.len();
        let mut total: u32 = 0;
        unsafe {
            let ptr = self.data.as_ptr();
            // Loop runs from 0 to len-1 (inclusive). When i equals len-1,
            // ptr.add(i + 1) accesses memory beyond the allocated buffer.
            for i in 0..len {
                let first = *ptr.add(i);
                let second = *ptr.add(i + 1); // Vulnerable: out-of-bound read when i+1 == len.
                total = total.wrapping_add(first).wrapping_add(second);
            }
        }
        total
    }
}

// A helper function that spawns a worker thread to perform computation.
fn spawn_task(holder: Arc<Mutex<DataHolder>>) {
    let handle = thread::spawn(move || {
        let guard = holder.lock().unwrap();
        let sum = guard.compute();
        println!("Computed value: {}", sum);
    });
    handle.join().unwrap();
}

fn main() {
    // Initialize the data buffer with 5 elements.
    let holder = Arc::new(Mutex::new(DataHolder { data: vec![1, 2, 3, 4, 5] }));
    spawn_task(holder.clone());
}