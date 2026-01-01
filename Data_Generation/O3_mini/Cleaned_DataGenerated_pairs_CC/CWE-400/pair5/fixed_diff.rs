use std::sync::{Arc, Mutex};
    inner: Mutex<Vec<u32>>,
    max_tasks: usize,
            inner: Mutex::new(Vec::new()),
            max_tasks: 10000, 
        let mut guard = self.inner.lock().expect("Mutex poisoned");
        if guard.len() < self.max_tasks {
            guard.push(task);
        let guard = self.inner.lock().expect("Mutex poisoned");
        guard.len()
        let mut guard = self.inner.lock().expect("Mutex poisoned");
        guard.clear();
    println!("Total tasks accumulated (capped): {}", queue.len());
