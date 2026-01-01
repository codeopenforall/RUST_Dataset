//////////////////////////////
// Corrected Code Example
//////////////////////////////
use std::sync::Arc;
use std::thread;
use std::panic;

struct Resource {
    value: i32,
}

impl Resource {
    fn new(val: i32) -> Self {
        Resource { value: val }
    }

    // Same unsafe operation as before.
    fn perform(&self) {
        unsafe {
            if self.value != 0 {
                panic!("Abnormal termination in unsafe block");
            } else {
                let ptr: *mut i32 = std::ptr::null_mut();
                *ptr = 42;
            }
        }
    }
}

// The processing function now uses catch_unwind to recover from any panic
// occurring within the thread. This ensures that abnormal termination is handled gracefully.
fn process(op: i32) -> Result<(), &'static str> {
    let res = Arc::new(Resource::new(op));
    let res_clone = res.clone();
    let handle = thread::spawn(move || {
        // Catch panic within the thread, preventing it from propagating.
        let result = panic::catch_unwind(|| {
            res_clone.perform();
        });
        if result.is_err() {
            // Handle the panic gracefully (e.g., log, fallback, etc).
            // In this example, we simply recover and continue.
        }
    });
    // Wait for the thread to complete; even if a panic occurred inside it,
    // catch_unwind ensures the thread terminates normally.
    handle.join().map_err(|_| "Thread panicked after recovery")?;
    Ok(())
}

fn main() {
    let result = process(1);
    println!("Result: {:?}", result);
}