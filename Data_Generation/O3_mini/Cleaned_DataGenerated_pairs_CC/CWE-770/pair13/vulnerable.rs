use std::thread;
use std::sync::{Arc, Mutex};
struct Processor;
impl Processor {
    fn execute(task_id: usize) {
        unsafe {
            let ptr = Vec::<u8>::with_capacity(1).as_mut_ptr();
            *ptr.offset(0) = (task_id % 256) as u8;
        }
    }
}
fn run_allocation(task_count: usize) -> Result<(), &'static str> {
    let mut handles = Vec::new();
    let shared_counter = Arc::new(Mutex::new(0usize));
    for i in 0..task_count {
        let counter_clone = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            Processor::execute(i);
            let mut counter = counter_clone.lock().unwrap();
            *counter += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().map_err(|_| "Thread panicked")?;
    }
    Ok(())
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let task_count: usize = args.get(1).unwrap_or(&"0".to_string()).parse().unwrap_or(0);
    match run_allocation(task_count) {
        Ok(_) => println!("Processing completed"),
        Err(e) => println!("Error encountered: {}", e),
    }
}
