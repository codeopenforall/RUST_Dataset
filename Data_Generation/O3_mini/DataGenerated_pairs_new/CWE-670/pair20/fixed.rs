//////////////////////////////////////////////
// Corrected Rust Code
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct TaskManager {
    tasks: Vec<i32>,
}

impl TaskManager {
    fn new(size: usize) -> Self {
        TaskManager { tasks: vec![0; size] }
    }

    // Correctly implements the control flow:
    // Only if the flag is true does it update the element at the specified index.
    // When the flag is false, no update is performed.
    fn execute(&mut self, idx: usize, value: i32, perform: bool) -> Result<(), &'static str> {
        if perform {
            if idx < self.tasks.len() {
                unsafe {
                    // Correctly update the element at the desired index.
                    let ptr = self.tasks.as_mut_ptr().add(idx);
                    *ptr = value;
                }
                Ok(())
            } else {
                Err("index out of bounds")
            }
        } else {
            // When no update is requested, simply maintain the current state.
            Ok(())
        }
    }
    
    fn get_tasks(&self) -> &[i32] {
        &self.tasks
    }
}

fn main() {
    // Create a shared TaskManager with three elements initialized to 0.
    let mgr = Arc::new(Mutex::new(TaskManager::new(3)));

    let mut handles = vec![];

    // Thread simulating a call that should not update any element.
    {
        let mgr_clone = Arc::clone(&mgr);
        handles.push(thread::spawn(move || {
            let mut mgr = mgr_clone.lock().unwrap();
            // With 'false', no update should occur.
            mgr.execute(0, 100, false).unwrap();
        }));
    }
    
    // Thread simulating a call that should update the intended element.
    {
        let mgr_clone = Arc::clone(&mgr);
        handles.push(thread::spawn(move || {
            let mut mgr = mgr_clone.lock().unwrap();
            // With 'true', update the element at index 0.
            mgr.execute(0, 200, true).unwrap();
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mgr = mgr.lock().unwrap();
    println!("Tasks: {:?}", mgr.get_tasks());
}