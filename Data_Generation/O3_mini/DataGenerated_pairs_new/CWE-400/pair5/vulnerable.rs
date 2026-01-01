//////////////////////////////////////////
// Vulnerable Code: Resource Exhaustion
//////////////////////////////////////////
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

pub struct WorkQueue {
    // Using UnsafeCell to bypass Rust’s normal aliasing rules.
    // The internal vector is not protected from unbounded growth.
    inner: UnsafeCell<Vec<u32>>,
}

unsafe impl Sync for WorkQueue {}

impl WorkQueue {
    pub fn new() -> Arc<Self> {
        Arc::new(WorkQueue {
            inner: UnsafeCell::new(Vec::new()),
        })
    }

    // Adds a task without enforcing any capacity limits.
    pub fn add_task(&self, task: u32) {
        unsafe {
            // Vulnerability: No check for uncontrolled growth.
            // Directly pushing to a vector with no backpressure.
            (*self.inner.get()).push(task);
        }
    }

    // Returns the current number of tasks.
    pub fn len(&self) -> usize {
        unsafe { (*self.inner.get()).len() }
    }

    // Process all tasks (dummy processing for illustration).
    pub fn process_all(&self) {
        unsafe {
            let tasks = &mut *self.inner.get();
            // Simulate processing by clearing the vector.
            tasks.clear();
        }
    }
}

fn main() {
    // Spawn multiple threads adding tasks in large quantities, simulating a DoS.
    let queue = WorkQueue::new();
    let mut handles = Vec::new();

    // Create 50 threads: each pushes 1000 tasks (50_000 tasks total).
    for i in 0..50 {
        let q = queue.clone();
        handles.push(thread::spawn(move || {
            for j in 0..1000 {
                // Each thread adds tasks without limit.
                q.add_task(i * 1000 + j);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Display the number of tasks accumulated.
    println!("Total tasks accumulated: {}", queue.len());

    // Process tasks.
    queue.process_all();
    println!("Tasks processed and cleared.");
}