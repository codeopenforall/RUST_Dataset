//////////////////////////////////////////
// Fixed Code: Controlled Resource Usage
//////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

pub struct WorkQueue {
    // The inner vector is now protected by a Mutex.
    // A maximum limit is enforced to prevent uncontrolled growth.
    inner: Mutex<Vec<u32>>,
    max_tasks: usize,
}

impl WorkQueue {
    pub fn new() -> Arc<Self> {
        Arc::new(WorkQueue {
            inner: Mutex::new(Vec::new()),
            max_tasks: 10000, // Set a threshold for maximum tasks.
        })
    }

    // Adds a task only if the current number of tasks is below the limit.
    pub fn add_task(&self, task: u32) {
        let mut guard = self.inner.lock().expect("Mutex poisoned");
        if guard.len() < self.max_tasks {
            guard.push(task);
        }
        // Else: silently drop the task to prevent resource exhaustion.
    }

    // Returns the current number of tasks.
    pub fn len(&self) -> usize {
        let guard = self.inner.lock().expect("Mutex poisoned");
        guard.len()
    }

    // Process all tasks (dummy processing for illustration).
    pub fn process_all(&self) {
        let mut guard = self.inner.lock().expect("Mutex poisoned");
        // Simulate processing by clearing the vector.
        guard.clear();
    }
}

fn main() {
    // Spawn multiple threads adding tasks, but with a limit in place.
    let queue = WorkQueue::new();
    let mut handles = Vec::new();

    // Create 50 threads: each attempts to push 1000 tasks.
    for i in 0..50 {
        let q = queue.clone();
        handles.push(thread::spawn(move || {
            for j in 0..1000 {
                // Tasks will only be added until the threshold is met.
                q.add_task(i * 1000 + j);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // The total tasks will not exceed the defined maximum.
    println!("Total tasks accumulated (capped): {}", queue.len());

    // Process tasks.
    queue.process_all();
    println!("Tasks processed and cleared.");
}