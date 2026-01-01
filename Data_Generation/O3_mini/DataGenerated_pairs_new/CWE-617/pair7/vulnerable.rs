///////////////////////////////////////////////
// Vulnerable Code Sample (Unsafe, Unsynchronized)
///////////////////////////////////////////////
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct DataHolder {
    // Public field to facilitate unsynchronized modification.
    pub data: Vec<u32>,
}

impl DataHolder {
    pub fn new(size: usize) -> Self {
        DataHolder { data: vec![0; size] }
    }

    // Uses an unsafe raw pointer to access the vector.
    pub unsafe fn get_ptr(&self) -> *const u32 {
        self.data.as_ptr()
    }

    // Reads the element at the specified index using unsafe pointer arithmetic.
    // The reached assertion is influenced by external concurrent modification.
    pub fn process_input(&self, idx: usize) {
        unsafe {
            let ptr = self.get_ptr();
            // The value is attacker-influenced by concurrent unsynchronized write.
            let value = *ptr.add(idx);
            // Reachable assertion that may be triggered by an attacker-controlled state.
            // If value equals the trigger value (42), the assertion fails.
            assert!(value != 42, "Invariant violated: unexpected trigger value encountered.");
        }
    }
}

// A critical function that executes the vulnerable workflow.
// It is public so that external tests can invoke it.
pub fn critical_run() {
    // Create a shared DataHolder with 10 elements.
    let holder = Arc::new(DataHolder::new(10));

    // Spawn a thread that unsafely modifies the underlying data.
    let holder_clone = Arc::clone(&holder);
    let modify_handle = thread::spawn(move || {
        // Wait a bit to simulate asynchronous and unsynchronized update.
        thread::sleep(Duration::from_millis(50));
        unsafe {
            // Directly obtain a mutable pointer to the vector’s data.
            let ptr = holder_clone.data.as_ptr() as *mut u32;
            // Unsynchronized write: set element at index 5 to the trigger value.
            *ptr.add(5) = 42;
        }
    });

    // Let the modifying thread run.
    thread::sleep(Duration::from_millis(100));
    // Process input based on an index that may have been compromised.
    holder.process_input(5);
    modify_handle.join().unwrap();
}

fn main() {
    // The main entry-point executing the critical run.
    critical_run();
}