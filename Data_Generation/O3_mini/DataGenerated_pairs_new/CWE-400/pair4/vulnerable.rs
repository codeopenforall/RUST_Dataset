///////////////////////////////////////////////////////////////////////////////
// Vulnerability: Uncontrolled Resource Consumption (CWE-400)
// Description: This code accepts tasks and appends them to an internal vector 
// without enforcing any upper bound. In the submit method an unsafe block is 
// used to manually insert the task into the vector without capacity or limit 
// checks. Under heavy load (many tasks), this design permits unbounded memory 
// growth that may lead to denial-of-service conditions.
///////////////////////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Service {
    tasks: Arc<Mutex<Vec<String>>>,
}

impl Service {
    fn new() -> Self {
        // Preallocate some capacity.
        Service {
            tasks: Arc::new(Mutex::new(Vec::with_capacity(1024))),
        }
    }

    // The submit method uses an unsafe block to perform raw pointer insertion 
    // into the internal vector. No upper bound is enforced.
    fn submit(&self, data: String) -> Result<(), String> {
        let mut list = self.tasks.lock().map_err(|_| "Lock poisoned".to_string())?;
        let len = list.len();
        // Unsafe block lacking bounds checking – may write beyond intended limits.
        unsafe {
            let ptr = list.as_mut_ptr();
            ptr.add(len).write(data);
            list.set_len(len + 1);
        }
        Ok(())
    }
}

fn main() {
    let svc = Service::new();

    // Spawn a background thread to simulate slow task consumption.
    {
        let tasks_clone = svc.tasks.clone();
        thread::spawn(move || {
            loop {
                {
                    let mut list = tasks_clone.lock().unwrap();
                    if !list.is_empty() {
                        // Simulate processing by removing one task.
                        list.remove(0);
                    }
                }
                thread::sleep(Duration::from_millis(50));
            }
        });
    }

    // In a high-load scenario, tasks are submitted in a tight loop without backpressure.
    // This loop deliberately sends 1100 tasks, overwhelming the service.
    for i in 0..1100 {
        if let Err(e) = svc.submit(format!("Task {}", i)) {
            eprintln!("Submission error: {}", e);
            break;
        }
    }
    println!("Submitted 1100 tasks");
}