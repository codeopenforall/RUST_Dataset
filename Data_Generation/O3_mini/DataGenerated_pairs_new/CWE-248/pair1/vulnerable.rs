use std::{thread};

fn run_app() -> Result<(), &'static str> {
    // Spawn a thread that executes an unsafe block without catching panics
    let handle = thread::spawn(|| {
        unsafe {
            // Simulate an unsafe FFI call that triggers a panic due to a null pointer check.
            trigger();
        }
    });

    // Joining the thread. If the thread panicked, the error will propagate.
    // This unwrap causes the application to abort if a panic occurs.
    handle.join().map_err(|_| "Thread panicked")?;
    Ok(())
}

unsafe fn trigger() {
    // Using a raw pointer to simulate potential FFI misuse.
    let ptr: *const i32 = std::ptr::null();
    if ptr.is_null() {
        // Trigger a panic which is not caught, leading to abnormal termination.
        panic!("Simulated panic: null pointer dereference");
    }
    // In a real scenario, pointer dereferencing and FFI calls would occur here.
}

fn main() {
    // The application will abort if run_app returns an error (i.e. if the panic propagates).
    run_app().unwrap();
}