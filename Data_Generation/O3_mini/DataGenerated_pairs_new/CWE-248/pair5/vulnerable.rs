/*
This code spawns a thread that calls an unsafe routine triggering a panic.
The panic is not caught within the thread, so when joining the thread,
the unwrap on the join result causes the entire process to abort.
*/
use std::{thread};

unsafe fn trigger(flag: bool) {
    if flag {
        // Panic occurs here, within an unsafe block.
        panic!("Abnormal termination encountered!");
    }
}

struct Executor;

impl Executor {
    fn new() -> Self {
        Self
    }
    fn run(&self) {
        // Unsafe call that might panic.
        unsafe {
            trigger(true)
        }
    }
}

pub fn run_app() {
    // Spawn a thread that runs the processing.
    let handle = thread::spawn(|| {
        let exec = Executor::new();
        exec.run();
    });
    // This join will propagate the panic from the thread,
    // causing an abnormal termination.
    let _ = handle.join().unwrap();
}

fn main() {
    run_app();
}