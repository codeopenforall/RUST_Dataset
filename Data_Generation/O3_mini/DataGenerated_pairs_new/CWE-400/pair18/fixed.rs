////////////////////////////////////////////////////////////
// Fixed Example: Controlled Resource Consumption with Mutex Protection
// CWE-400 Mitigation – Prevent Unbounded Resource Consumption
//
// This version re-implements the resource pool using a Mutex<Vec<i32>> and imposes
// an explicit upper bound (MAX_CAPACITY) on the number of elements. The insert method
// locks the mutex and only pushes new values if the capacity remains below the threshold,
// preventing uncontrolled resource consumption.
////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

const MAX_CAPACITY: usize = 1000;

pub struct Pool {
    data: Mutex<Vec<i32>>,
}

impl Pool {
    pub fn new() -> Self {
        Pool {
            data: Mutex::new(Vec::new()),
        }
    }

    pub fn insert(&self, value: i32) {
        let mut vec = self.data.lock().unwrap();
        if vec.len() < MAX_CAPACITY {
            vec.push(value);
        }
    }

    pub fn count(&self) -> usize {
        let vec = self.data.lock().unwrap();
        vec.len()
    }
}

pub fn simulate() -> bool {
    // Spawn 4 threads that concurrently add up to 1000 elements each.
    let pool = Arc::new(Pool::new());
    let mut handles = Vec::new();

    for _ in 0..4 {
        let pool_clone = Arc::clone(&pool);
        let handle = thread::spawn(move || {
            for i in 0..1000 {
                pool_clone.insert(i);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // With the capacity check enforced, the pool cannot exceed MAX_CAPACITY elements.
    let count = pool.count();
    count <= MAX_CAPACITY
}

fn main() {
    let safe = simulate();
    if safe {
        println!("Resource usage within safe limits: {}", safe);
    } else {
        println!("Resource usage exceeded safe limits: {}", safe);
    }
}