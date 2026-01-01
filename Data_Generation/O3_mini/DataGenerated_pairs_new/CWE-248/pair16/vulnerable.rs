////////////////// Vulnerable Version //////////////////
use std::thread;

struct Processor;

impl Processor {
    fn execute(&self) {
        let handle = thread::spawn(|| {
            // Unsafe block with raw pointer manipulation.
            unsafe {
                let mut data: i32 = 10;
                let ptr: *mut i32 = &mut data;
                // Adjust the value.
                *ptr = *ptr - 10;
                // If the computed value is zero, trigger a panic.
                // This panic is not caught and propagates through the thread join.
                if *ptr == 0 {
                    panic!("Division by zero due to invalid operation");
                }
            }
        });
        // Joining the thread will propagate any panic produced inside.
        // No unwind catch is applied.
        let _ = handle.join().unwrap();
    }
}

fn run() {
    let proc = Processor;
    proc.execute();
}

fn main() {
    run();
}
/////////////////////////////////////////////////////////