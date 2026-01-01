////////////////////////////////////////////////////////////
// Vulnerability Example: Uncontrolled Resource Consumption
// CWE-400: DoS – Unbounded resource usage via unsynchronized, unsafe concurrent writes.
//
// This code defines a pool that is backed by an unsafe container (UnsafeCell<Vec<i32>>)
// which is shared across threads via Arc. The insert method uses an unsafe block to
// push items into the Vec without any synchronization mechanism or capacity check.
// When multiple threads concurrently call insert in a loop, the vector grows without
// bounds. This can allow uncontrolled memory consumption leading to a denial-of-service
// (DoS) condition.
////////////////////////////////////////////////////////////
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

pub struct Pool {
    data: UnsafeCell<Vec<i32>>,
}

// Mark our type as Sync even though it contains an UnsafeCell. This is unsafe
// because concurrent unsynchronized writes occur.
unsafe impl Sync for Pool {}

impl Pool {
    pub fn new() -> Self {
        Pool {
            data: UnsafeCell::new(Vec::new()),
        }
    }

    pub fn insert(&self, value: i32) {
        // Vulnerable: Unprotected push to the vector in an unsafe block.
        unsafe {
            (*self.data.get()).push(value);
        }
    }

    pub fn count(&self) -> usize {
        unsafe { (*self.data.get()).len() }
    }
}

pub fn simulate() -> bool {
    // Spawn 4 threads that concurrently add 1000 elements each.
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

    // In the vulnerable scenario, no capacity is enforced.
    // The expected safe threshold is 1000 elements, but here the pool will hold 4000.
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