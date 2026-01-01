//////////////////////////////
// Vulnerable Code Sample   //
// CWE-248: Uncaught Panic    //
//////////////////////////////
use std::thread;

struct Handler;

impl Handler {
    fn new() -> Self {
        Self {}
    }

    // This method spawns a thread that executes an unsafe block.
    // The unsafe block intentionally triggers a panic without proper handling.
    fn execute(&self) {
        // Spawn a thread that executes unsafe code which may panic.
        let th = thread::spawn(|| {
            unsafe {
                let mut value: i32 = 0;
                let raw_ptr: *mut i32 = &mut value;
                // The following condition is a placeholder for an unexpected condition.
                // Dereferencing the raw pointer without appropriate checks.
                if *raw_ptr == 0 {
                    // Panic is triggered inside an unsafe context.
                    panic!("abnormal termination: unexpected condition encountered");
                }
            }
        });
        // Thread panic is not caught; unwrap may propagate the panic across threads.
        th.join().unwrap();
    }
}

fn run_system() {
    let handler = Handler::new();
    handler.execute();
}

fn main() {
    // Calling run_system() will result in an uncaught panic from the unsafe thread.
    run_system();
}