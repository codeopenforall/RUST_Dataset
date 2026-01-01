use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
struct Data {
    value: UnsafeCell<u32>,
}
unsafe impl Sync for Data {}
impl Data {
    fn new(val: u32) -> Self {
        Data {
            value: UnsafeCell::new(val),
        }
    }
    fn update(&self) {
        unsafe {
            let ptr = self.value.get();
            let current = ptr.read(); 
            thread::sleep(Duration::from_micros(1));
            ptr.write(current + 1); 
        }
    }
    fn fetch(&self) -> u32 {
        unsafe { self.value.get().read() }
    }
}
fn main() {
    let shared = Arc::new(Data::new(0));
    let mut workers = Vec::new();
    for _ in 0..10 {
        let handler = {
            let local = Arc::clone(&shared);
            thread::spawn(move || {
                for _ in 0..1000 {
                    local.update();
                }
            })
        };
        workers.push(handler);
    }
    for worker in workers {
        worker.join().expect("Thread panicked");
    }
    println!("Final value: {}", shared.fetch());
}
