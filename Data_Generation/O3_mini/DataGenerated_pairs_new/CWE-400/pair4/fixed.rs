///////////////////////////////////////////////////////////////////////////////
// Fixed: Controlled Resource Consumption (CWE-400)
// Description: This corrected code introduces an upper bound to the task queue.
// The submit method now checks the current queue length against a defined limit,
// returning an error when the limit is reached. The unsafe block is still used for
// low-level insertion but only after confirming that the request is within bounds.
///////////////////////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const MAX_TASKS: usize = 1000;

struct Service {
    tasks: Arc<Mutex<Vec<String>>>,
}

impl Service {
    fn new() -> Self {
        Service {
            tasks: Arc::new(Mutex::new(Vec::with_capacity(1024))),
        }
    }

    // The submit method now enforces a maximum number of tasks.
    fn submit(&self, data: String) -> Result<(), String> {
        let mut list = self.tasks.lock().map_err(|_| "Lock poisoned".to_string())?;
        if list.len() >= MAX_TASKS {
            return Err("Queue limit reached".to_string());
        }
        let len = list.len();
        unsafe {
            let ptr = list.as_mut_ptr();
            // Safe because we reserved enough capacity and enforce a limit.
            ptr.add(len).write(data);
            list.set_len(len + 1);
        }
        Ok(())
    }
}

fn main() {
    let svc = Service::new();

    // Background thread simulating slow consumption.
    {
        let tasks_clone = svc.tasks.clone();
        thread::spawn(move || {
            loop {
                {
                    let mut list = tasks_clone.lock().unwrap();
                    if !list.is_empty() {
                        list.remove(0);
                    }
                }
                thread::sleep(Duration::from_millis(50));
            }
        });
    }

    // Submit tasks; once the fixed maximum is reached, submission will error out.
    for i in 0..1100 {
        if let Err(e) = svc.submit(format!("Task {}", i)) {
            eprintln!("Submission error: {}", e);
            break;
        }
    }
    println!("Tasks submission complete (max limit enforced)");
}