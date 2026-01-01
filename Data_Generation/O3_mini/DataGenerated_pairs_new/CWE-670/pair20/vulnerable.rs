//////////////////////////////////////////////
// Vulnerable Rust Code
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

    // This function is intended to perform an update only if the flag is true.
    // However, due to flawed control flow the branch for flag==false also writes,
    // and the branch for flag==true writes to the wrong index.
    fn execute(&mut self, idx: usize, value: i32, perform: bool) -> Result<(), &'static str> {
        unsafe {
            match perform {
                true => {
                    if idx < self.tasks.len() - 1 {
                        // BUG: Instead of updating the desired element at idx,
                        // it advances one extra slot (idx + 1), violating the invariant.
                        let ptr = self.tasks.as_mut_ptr().add(idx + 1);
                        *ptr = value;
                        Ok(())
                    } else {
                        Err("index out of bounds")
                    }
                },
                false => {
                    if idx < self.tasks.len() {
                        // BUG: Even when no update is requested, it erroneously
                        // writes the value to the element at idx.
                        let ptr = self.tasks.as_mut_ptr().add(idx);
                        *ptr = value;
                        Ok(())
                    } else {
                        Err("index out of bounds")
                    }
                },
            }
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

    // Thread simulating a call that should not update the element.
    {
        let mgr_clone = Arc::clone(&mgr);
        handles.push(thread::spawn(move || {
            let mut mgr = mgr_clone.lock().unwrap();
            // Intention: With 'false', nothing should be updated.
            // Vulnerability: It erroneously writes 100 to index 0.
            mgr.execute(0, 100, false).unwrap();
        }));
    }
    
    // Thread simulating a call that should update the desired element.
    {
        let mgr_clone = Arc::clone(&mgr);
        handles.push(thread::spawn(move || {
            let mut mgr = mgr_clone.lock().unwrap();
            // Intention: With 'true', update at index 0.
            // Vulnerability: It erroneously writes 200 to index (0+1), i.e. index 1.
            mgr.execute(0, 200, true).unwrap();
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mgr = mgr.lock().unwrap();
    println!("Tasks: {:?}", mgr.get_tasks());
}