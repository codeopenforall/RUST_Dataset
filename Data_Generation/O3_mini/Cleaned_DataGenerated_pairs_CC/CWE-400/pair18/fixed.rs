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
