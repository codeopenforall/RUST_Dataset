use std::thread;
use std::panic;
struct Processor;
impl Processor {
    fn execute(&self, trigger: bool) -> Result<(), &'static str> {
        let handle = thread::spawn(move || {
            let result = panic::catch_unwind(|| {
                if trigger {
                    panic!("Critical failure encountered in concurrent task");
                }
                println!("Task completed without incident.");
            });
            result
        });
        match handle.join() {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(_)) => Err("Thread encountered a panic"),
            Err(_) => Err("Failed to join thread"),
        }
    }
}
fn main() {
    let service = Processor;
    match service.execute(true) {
        Ok(_) => println!("Execution completed successfully."),
        Err(e) => println!("Execution error handled: {}", e),
    }
}
