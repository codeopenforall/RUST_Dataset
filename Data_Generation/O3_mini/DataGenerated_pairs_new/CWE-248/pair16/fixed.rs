////////////////// Fixed Version //////////////////
use std::thread;
use std::panic;

struct Processor;

impl Processor {
    fn execute(&self) -> Result<(), &'static str> {
        let handle = thread::spawn(|| {
            // Unsafe block with raw pointer manipulation processed in a thread.
            unsafe {
                let mut data: i32 = 10;
                let ptr: *mut i32 = &mut data;
                *ptr = *ptr - 10;
                if *ptr == 0 {
                    panic!("Division by zero due to invalid operation");
                }
            }
        });
        // Instead of directly unwrapping, we catch any panic from the thread join.
        // This prevents panic propagation and allows controlled error handling.
        match handle.join() {
            Ok(_) => Ok(()),
            Err(_) => Err("Recovered from thread panic"),
        }
    }
}

fn run() -> Result<(), &'static str> {
    let proc = Processor;
    proc.execute()
}

fn main() {
    // Instead of letting a panic crash the program, we check for errors in run.
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
//////////////////////////////////////////////////////