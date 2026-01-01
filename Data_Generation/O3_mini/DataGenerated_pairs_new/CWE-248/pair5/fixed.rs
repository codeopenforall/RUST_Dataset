/*
In this corrected version, any panic arising from the unsafe operation is caught
using catch_unwind. This prevents the panic from propagating and causing the thread
to abort unexpectedly. The error is logged, and the main thread continues normally.
*/
use std::{thread, panic};

unsafe fn trigger(flag: bool) {
    if flag {
        panic!("Abnormal termination encountered!");
    }
}

struct Executor;

impl Executor {
    fn new() -> Self {
        Self
    }
    fn run(&self) -> Result<(), &'static str> {
        // Wrap the unsafe operation with catch_unwind to safely handle panic.
        let result = panic::catch_unwind(|| unsafe {
            trigger(true)
        });
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Recovered from unsafe panic"),
        }
    }
}

pub fn run_app() {
    let handle = thread::spawn(|| {
        let exec = Executor::new();
        if let Err(e) = exec.run() {
            eprintln!("Error: {}", e);
        }
    });
    // Use join without unwrap; errors have been locally handled.
    let _ = handle.join();
}

fn main() {
    run_app();
}