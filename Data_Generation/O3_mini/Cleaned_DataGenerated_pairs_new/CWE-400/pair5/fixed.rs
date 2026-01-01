use std::sync::{Arc, Mutex};
use std::thread;
pub struct WorkQueue {
    inner: Mutex<Vec<u32>>,
    max_tasks: usize,
}
impl WorkQueue {
    pub fn new() -> Arc<Self> {
        Arc::new(WorkQueue {
            inner: Mutex::new(Vec::new()),
            max_tasks: 10000, 
        })
    }
    pub fn add_task(&self, task: u32) {
        let mut guard = self.inner.lock().expect("Mutex poisoned");
        if guard.len() < self.max_tasks {
            guard.push(task);
        }
    }
    pub fn len(&self) -> usize {
        let guard = self.inner.lock().expect("Mutex poisoned");
        guard.len()
    }
    pub fn process_all(&self) {
        let mut guard = self.inner.lock().expect("Mutex poisoned");
        guard.clear();
    }
}
fn main() {
    let queue = WorkQueue::new();
    let mut handles = Vec::new();
    for i in 0..50 {
        let q = queue.clone();
        handles.push(thread::spawn(move || {
            for j in 0..1000 {
                q.add_task(i * 1000 + j);
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total tasks accumulated (capped): {}", queue.len());
    queue.process_all();
    println!("Tasks processed and cleared.");
}
