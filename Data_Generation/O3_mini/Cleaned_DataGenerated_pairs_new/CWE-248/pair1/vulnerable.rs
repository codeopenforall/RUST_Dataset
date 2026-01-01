use std::{thread};
fn run_app() -> Result<(), &'static str> {
    let handle = thread::spawn(|| {
        unsafe {
            trigger();
        }
    });
    handle.join().map_err(|_| "Thread panicked")?;
    Ok(())
}
unsafe fn trigger() {
    let ptr: *const i32 = std::ptr::null();
    if ptr.is_null() {
        panic!("Simulated panic: null pointer dereference");
    }
}
fn main() {
    run_app().unwrap();
}
