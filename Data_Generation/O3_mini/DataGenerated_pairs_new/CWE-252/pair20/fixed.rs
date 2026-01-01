--------------------------------------------------
use std::env;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use std::ffi::c_void;

struct Executor;

impl Executor {
    // This function runs several concurrent tasks and properly handles errors
    // coming from an unsafe operation.
    pub fn run(input: &str) -> Result<(), String> {
        let shared_counter = Arc::new(Mutex::new(0));
        let (tx, rx) = mpsc::channel();
        let mut threads = vec![];

        // Spawn threads that perform an unsafe operation
        for i in 0..4 {
            let counter = Arc::clone(&shared_counter);
            let param = input.to_owned();
            let thread_tx = tx.clone();
            let handle = thread::spawn(move || {
                // Perform the unsafe operation and check its result.
                let op_result = unsafe { risky_operation(i, &param) };
                // Report the outcome of the unsafe operation to the main thread.
                if let Err(e) = op_result {
                    // Send the error message and return early from the thread.
                    let _ = thread_tx.send(Err(e.to_owned()));
                    return;
                } else {
                    let _ = thread_tx.send(Ok(()));
                }
                // Update the shared state only upon success.
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            threads.push(handle);
        }

        // Drop the original transmitter to avoid waiting for extra messages.
        drop(tx);

        // Check if any thread reported an error.
        for result in rx {
            if let Err(msg) = result {
                // If any error is reported, join all threads and return an error.
                for th in threads {
                    let _ = th.join();
                }
                return Err(format!("A task failed: {}", msg));
            }
        }

        // Ensure all threads are joined.
        for th in threads {
            let _ = th.join();
        }

        // Final invariant check.
        if *shared_counter.lock().unwrap() == 4 {
            Ok(())
        } else {
            Err("Not all tasks completed correctly".to_owned())
        }
    }
}

// Unsafe function simulating an operation that may fail.
unsafe fn risky_operation(task_id: u32, param: &str) -> Result<(), &'static str> {
    // Introduce a slight delay to simulate a real operation.
    if param.contains("fail") && task_id == 2 {
        Err("Operation failed in unsafe block")
    } else {
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

    // Now check the result of run and exit accordingly.
    match Executor::run(&input) {
        Ok(()) => println!("Execution finished successfully."),
        Err(e) => println!("Execution encountered an error: {}", e),
    }
}
--------------------------------------------------