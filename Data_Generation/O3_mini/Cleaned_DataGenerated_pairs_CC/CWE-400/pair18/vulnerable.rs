use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
pub struct Pool {
    data: UnsafeCell<Vec<i32>>,
}
unsafe impl Sync for Pool {}
impl Pool {
    pub fn new() -> Self {
        Pool {
            data: UnsafeCell::new(Vec::new()),
        }
    }
    pub fn insert(&self, value: i32) {
        unsafe {
            (*self.data.get()).push(value);
        }
    }
    pub fn count(&self) -> usize {
        unsafe { (*self.data.get()).len() }
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
    count <= 1000
}
fn main() {
    let safe = simulate();
    if safe {
        println!("Resource usage within safe limits: {}", safe);
    } else {
        println!("Resource usage exceeded safe limits: {}", safe);
    }
}
