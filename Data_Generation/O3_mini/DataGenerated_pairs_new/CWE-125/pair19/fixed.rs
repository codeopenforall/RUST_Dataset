//////////////////////////////////////////////////
// CWE-125 Demonstration: Safe Buffer Access Fix  //
// This code corrects the out-of-bound read issue//
// by limiting the loop to valid index pairs       //
//////////////////////////////////////////////////

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
    // This method safely computes a sum over adjacent pairs.
    // It ensures that the loop iterates only until the second-to-last element,
    // hence eliminating any possibility of out-of-bound access.
    fn compute(&self) -> u32 {
        let len = self.data.len();
        if len < 2 {
            return 0;
        }
        let mut total: u32 = 0;
        // Iterate only up to len - 1 so that index (i + 1) remains valid.
        for i in 0..(len - 1) {
            let first = self.data[i];
            let second = self.data[i + 1];
            total = total.wrapping_add(first).wrapping_add(second);
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