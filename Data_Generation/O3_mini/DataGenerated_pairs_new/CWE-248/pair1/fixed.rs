use std::{thread, panic};

fn run_app() -> Result<(), &'static str> {
    // Spawn a thread that executes an unsafe block, but catch any panics gracefully.
    let handle = thread::spawn(|| {
        // Wrap the unsafe call in catch_unwind to prevent panics from escaping the thread.
        let result = panic::catch_unwind(|| {
            unsafe { trigger(); }
        });
        if result.is_err() {
            // Handle the panic gracefully if needed (e.g., log the error).
            // The error is caught and suppressed to allow the application to continue.
        }
    });

    // Join the thread normally; even if the unsafe call panicked, it has been caught.
    handle.join().map_err(|_| "Thread join failed")?;
    Ok(())
}

unsafe fn trigger() {
    // Simulate an unsafe FFI call using a raw pointer.
    let ptr: *const i32 = std::ptr::null();
    if ptr.is_null() {
        // Trigger a panic to simulate error conditions in FFI calls.
        panic!("Simulated panic: null pointer dereference");
    }
}

fn main() {
    // The corrected application handles panics internally, so run_app() returns Ok.
    run_app().expect("Application encountered an error");
}