///////////////////////////////////////////////
// Corrected Code Sample (Safe, Synchronized)
///////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct DataHolder {
    // Use a mutex to safely synchronize access to the vector.
    data: Mutex<Vec<u32>>,
}

impl DataHolder {
    pub fn new(size: usize) -> Self {
        DataHolder { data: Mutex::new(vec![0; size]) }
    }

    // Safely processes input by acquiring the lock on the data.
    pub fn process_input(&self, idx: usize) {
        let guard = self.data.lock().expect("Lock poisoned");
        // Use Vec’s safe API to get the element; if out of bounds, default to 0.
        let value = guard.get(idx).copied().unwrap_or(0);
        // Instead of asserting on attacker-influenced state,
        // check and handle the trigger value without panicking.
        if value == 42 {
            // Instead of causing a denial-of-service via panic, log the event.
            eprintln!("Invariant violated: trigger value encountered; handling error gracefully.");
            return;
        }
    }
}

// A critical function that executes the corrected workflow.
// It is public so that external tests can invoke it.
pub fn critical_run() {
    // Create a shared DataHolder with safe synchronization.
    let holder = Arc::new(DataHolder::new(10));

    // Spawn a thread that safely modifies the underlying data after locking.
    let holder_clone = Arc::clone(&holder);
    let modify_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut guard = holder_clone.data.lock().expect("Lock poisoned");
        // Set element at index 5 to the trigger value in a controlled manner.
        guard[5] = 42;
    });

    thread::sleep(Duration::from_millis(100));
    // Process the input; the function now handles the trigger value gracefully.
    holder.process_input(5);
    modify_handle.join().unwrap();
}

fn main() {
    // The main entry-point executing the corrected run.
    critical_run();
}