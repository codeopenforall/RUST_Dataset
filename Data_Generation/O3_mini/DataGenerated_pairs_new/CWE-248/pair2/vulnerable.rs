//////////////////////// Vulnerable Version ////////////////////////
use std::thread;

struct Processor;

impl Processor {
    // This method spawns a new thread to simulate a critical task.
    // In case the task encounters a condition that should trigger an error,
    // the thread panics. However, the panic is not caught.
    fn execute(&self, trigger: bool) {
        unsafe {
            // Spawn a thread that will panic if trigger is true.
            let handle = thread::spawn(move || {
                if trigger {
                    panic!("Critical failure encountered in concurrent task");
                }
                // Simulate normal processing.
                println!("Task completed without incident.");
            });
            // Joining the thread without catching unwinding will propagate the panic.
            handle.join().unwrap();
        }
    }
}

fn main() {
    let service = Processor;
    // In this demo the trigger is set to true causing a panic inside the spawned thread.
    service.execute(true);
}