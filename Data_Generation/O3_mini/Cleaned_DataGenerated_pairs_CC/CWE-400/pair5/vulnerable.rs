use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
pub struct WorkQueue {
    inner: UnsafeCell<Vec<u32>>,
}
unsafe impl Sync for WorkQueue {}
impl WorkQueue {
    pub fn new() -> Arc<Self> {
        Arc::new(WorkQueue {
            inner: UnsafeCell::new(Vec::new()),
        })
    }
    pub fn add_task(&self, task: u32) {
        unsafe {
            (*self.inner.get()).push(task);
        }
    }
    pub fn len(&self) -> usize {
        unsafe { (*self.inner.get()).len() }
    }
    pub fn process_all(&self) {
        unsafe {
            let tasks = &mut *self.inner.get();
            tasks.clear();
        }
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
    println!("Total tasks accumulated: {}", queue.len());
    queue.process_all();
    println!("Tasks processed and cleared.");
}
