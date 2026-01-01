use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
struct SharedAccum {
    data: UnsafeCell<u32>,
}
unsafe impl Sync for SharedAccum {} 
impl SharedAccum {
    fn new(val: u32) -> Self {
        SharedAccum { data: UnsafeCell::new(val) }
    }
    fn update(&self, add: u32) {
        unsafe {
            let ptr = self.data.get();
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
