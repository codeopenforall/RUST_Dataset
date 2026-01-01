use std::thread;
use std::cell::UnsafeCell;
struct Counter {
    value: UnsafeCell<i32>,
}
unsafe impl Sync for Counter {}
impl Counter {
    fn new() -> Self {
        Counter { value: UnsafeCell::new(0) }
    }
    fn get(&self) -> i32 {
        unsafe { *self.value.get() }
    }
    fn add(&self, delta: i32) {
        unsafe {
            let cur = *self.value.get();
            *self.value.get() = cur + delta;
        }
    }
}
fn run() -> i32 {
    let counter = Counter::new();
    let mut threads = vec![];
    for _ in 0..10 {
        let c = &counter;
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                c.add(1);
            }
        });
        threads.push(handle);
    }
    for t in threads {
        t.join().unwrap();
    }
    counter.get()
}
fn main() {
    let result = run();
    println!("Final count is: {}", result);
}
