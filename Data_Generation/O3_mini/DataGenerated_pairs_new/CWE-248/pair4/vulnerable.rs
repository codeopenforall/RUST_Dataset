//////////////////////////////
// Vulnerable Code Example
//////////////////////////////
use std::sync::Arc;
use std::thread;

struct Resource {
    value: i32,
}

impl Resource {
    fn new(val: i32) -> Self {
        Resource { value: val }
    }

    // This method simulates an unsafe operation.
    // If the value is non-zero, it triggers a panic within an unsafe block.
    fn perform(&self) {
        unsafe {
            // Vulnerability trigger: abnormal termination without unwind handling.
            if self.value != 0 {
                // The panic here is not caught and will propagate abnormally.
                panic!("Abnormal termination in unsafe block");
            } else {
                // Simulated misuse of raw pointer leading to undefined behavior.
                let ptr: *mut i32 = std::ptr::null_mut();
                *ptr = 42;
            }
        }
    }
}

// The function spawns a thread that executes the unsafe operation.
// It does not catch panics within the spawned thread, leading to an abnormal termination.
fn process(op: i32) -> Result<(), &'static str> {
    let res = Arc::new(Resource::new(op));
    let res_clone = res.clone();
    let handle = thread::spawn(move || {
        res_clone.perform();
    });
    // Joining the thread: if the spawned thread panicked,
    // the error is propagated without recovery.
    handle.join().map_err(|_| "Thread panicked")
}

fn main() {
    let result = process(1);
    println!("Result: {:?}", result);
}