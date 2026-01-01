use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
struct SharedState {
    flag: UnsafeCell<bool>,
}
unsafe impl Sync for SharedState {}
impl SharedState {
    fn new() -> Self {
        SharedState { flag: UnsafeCell::new(false) }
    }
    unsafe fn check_then_set(&self) -> bool {
        if !*self.flag.get() {
            thread::sleep(Duration::from_millis(1));
            *self.flag.get() = true;
            true
        } else {
            false
        }
    }
}
fn run_test() -> usize {
    let shared = Arc::new(SharedState::new());
    let mut handles = Vec::new();
    for _ in 0..10 {
        let s = shared.clone();
        handles.push(thread::spawn(move || unsafe { s.check_then_set() }));
    }
    let results: Vec<bool> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    let success_count = results.into_iter().filter(|&x| x).count();
    println!("Number of successful accesses: {}", success_count);
    success_count
}
fn main() {
    run_test();
}
