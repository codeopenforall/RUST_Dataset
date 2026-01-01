/* 
   A shared accumulator that is updated concurrently.
   This version contains an integer overflow vulnerability in the update method.
   It uses an UnsafeCell to enable mutation across threads without proper synchronization
   against arithmetic overflow.
*/
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct SharedAccum {
    data: UnsafeCell<u32>,
}

unsafe impl Sync for SharedAccum {} // Allow sharing across threads

impl SharedAccum {
    fn new(val: u32) -> Self {
        SharedAccum { data: UnsafeCell::new(val) }
    }
    
    fn update(&self, add: u32) {
        // potential integer overflow vulnerability: unchecked wrapping addition.
        unsafe {
            let ptr = self.data.get();
            // Vulnerable unchecked addition (overflow vulnerability)
            *ptr = (*ptr).wrapping_add(add);
        }
    }
    
    fn get(&self) -> u32 {
        unsafe {
            *self.data.get()
        }
    }
}

pub fn compute_final() -> u32 {
    let accumulator = Arc::new(SharedAccum::new(0));
    let threads: Vec<_> = (0..4).map(|_| {
        let acc = accumulator.clone();
        thread::spawn(move || {
            // Each thread performs many updates that may trigger overflow.
            for _ in 0..1000 {
                // Using a large value to push the sum beyond u32::MAX.
                acc.update(10_000_000);
            }
        })
    }).collect();

    for t in threads {
        t.join().unwrap();
    }
    accumulator.get()
}

fn run_app() {
    let result = compute_final();
    println!("Final result: {}", result);
}

fn main() {
    run_app();
}