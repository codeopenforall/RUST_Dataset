/* 
   A corrected version of the shared accumulator.
   This version prevents integer overflows by using checked addition.
   If an update operation would overflow, the accumulator is set to u32::MAX,
   ensuring that the final state is within the valid range.
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
        unsafe {
            let ptr = self.data.get();
            // Fixed: use checked addition to detect overflow.
            // If an overflow is detected, we saturate the value to u32::MAX.
            match (*ptr).checked_add(add) {
                Some(sum) => *ptr = sum,
                None => *ptr = u32::MAX,
            }
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
            for _ in 0..1000 {
                // Using the same update value as the vulnerable version.
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