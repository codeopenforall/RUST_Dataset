use std::cell::UnsafeCell;
use std::sync::Arc;
    inner: UnsafeCell<Vec<u32>>,
unsafe impl Sync for WorkQueue {}
            inner: UnsafeCell::new(Vec::new()),
        unsafe {
            (*self.inner.get()).push(task);
        unsafe { (*self.inner.get()).len() }
        unsafe {
            let tasks = &mut *self.inner.get();
            tasks.clear();
        }
    println!("Total tasks accumulated: {}", queue.len());
