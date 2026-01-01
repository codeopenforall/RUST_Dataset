--------------------------------------------------
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::ffi::c_void;

struct Executor;

impl Executor {
    // This function runs several concurrent tasks.
    // Each task performs an unsafe operation that may return an error.
    // The error returned by the unsafe operation is ignored.
    pub fn run(input: &str) -> Result<(), String> {
        let shared_counter = Arc::new(Mutex::new(0));
        let mut threads = vec![];

        // Spawn a few threads performing an unsafe operation.
        for i in 0..4 {
            let counter = Arc::clone(&shared_counter);
            let param = input.to_owned();
            let handle = thread::spawn(move || {
                unsafe {
                    // The unsafe function returns a Result indicating an error,
                    // but here the returned error is ignored.
                    // If the parameter contains "fail" and i==2, an error is returned.
                    let res = risky_operation(i, &param);
                    let _ = res; // <-- Vulnerability: unchecked return value
                }
                // Regardless of the result of the unsafe operation, update the counter.
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            threads.push(handle);
        }

        // Wait for all threads.
        for th in threads {
            let _ = th.join();
        }

        // Verify an invariant that the shared counter reached the expected value.
        if *shared_counter.lock().unwrap() == 4 {
            Ok(())
        } else {
            Err("Not all tasks completed correctly".to_owned())
        }
    }
}

// Unsafe function simulating an operation that may fail.
unsafe fn risky_operation(task_id: u32, param: &str) -> Result<(), &'static str> {
    // Simulate a delay and then pretend an error occurred for a specific condition.
    if param.contains("fail") && task_id == 2 {
        // Simulate an error scenario.
        Err("Operation failed in unsafe block")
    } else {
        // Simulate a correct behavior.
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        "pass".to_owned()
    };

    // The return value from run() is not checked anywhere.
    // In case of a failure in one thread's unsafe call, the error is ignored.
    let _ = Executor::run(&input);
    println!("Execution finished.");
}
--------------------------------------------------