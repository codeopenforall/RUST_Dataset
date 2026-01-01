//////////////////////////////////////////////////////////////
// This code spawns a thread that executes an unsafe block,
// reading from a raw pointer, and if the trigger value (42)
// is encountered it panics without proper handling.
// The thread’s result is obtained via join().unwrap(), which
// will propagate an uncaught panic (i.e. abnormal termination)
// when the unsafe block calls panic. This simulates CWE-248.
//////////////////////////////////////////////////////////////
use std::thread;

pub struct Manager;

impl Manager {
    pub fn process(&self, value: i32) -> Result<i32, &'static str> {
        let handle = thread::spawn(move || -> i32 {
            // Using an unsafe block to simulate low-level memory access.
            unsafe {
                let ptr = &value as *const i32;
                let temp = *ptr;
                if temp == 42 {
                    // Trigger abnormal termination without catching
                    panic!("Triggered panic in unsafe block");
                }
                temp * 2
            }
        });
        // Vulnerability: Not catching panics. Using unwrap() will
        // cause the entire thread to panic if the spawned thread panics.
        let res = handle.join().unwrap();
        Ok(res)
    }
}

fn main() {
    let manager = Manager;
    // Using the trigger value to simulate the failure case.
    // Because process(42) will panic in the spawned thread,
    // unwrap() will abort execution via an uncaught panic.
    let _ = manager.process(42).unwrap();
    println!("Completed execution");
}