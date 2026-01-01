use std::{thread, panic};
fn run_app() -> Result<(), &'static str> {
    let handle = thread::spawn(|| {
        let result = panic::catch_unwind(|| {
            unsafe { trigger(); }
        });
        if result.is_err() {
        }
    });
    handle.join().map_err(|_| "Thread join failed")?;
    Ok(())
}
unsafe fn trigger() {
    let ptr: *const i32 = std::ptr::null();
    if ptr.is_null() {
        panic!("Simulated panic: null pointer dereference");
    }
}
fn main() {
    run_app().expect("Application encountered an error");
}
