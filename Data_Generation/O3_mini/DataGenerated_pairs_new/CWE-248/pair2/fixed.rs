//////////////////////// Fixed Version ////////////////////////
use std::thread;
use std::panic;

struct Processor;

impl Processor {
    // This method spawns a new thread and properly handles panics using catch_unwind.
    // It returns a Result based on whether the thread encountered a panic.
    fn execute(&self, trigger: bool) -> Result<(), &'static str> {
        // Spawn a thread that wraps its work in a catch_unwind block.
        let handle = thread::spawn(move || {
            let result = panic::catch_unwind(|| {
                if trigger {
                    panic!("Critical failure encountered in concurrent task");
                }
                println!("Task completed without incident.");
            });
            result
        });

        // Join the thread and handle any potential panics.
        match handle.join() {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(_)) => Err("Thread encountered a panic"),
            Err(_) => Err("Failed to join thread"),
        }
    }
}

fn main() {
    let service = Processor;
    // The trigger is set to true; however, the panic is caught,
    // so main continues and prints an error message instead of crashing.
    match service.execute(true) {
        Ok(_) => println!("Execution completed successfully."),
        Err(e) => println!("Execution error handled: {}", e),
    }
}