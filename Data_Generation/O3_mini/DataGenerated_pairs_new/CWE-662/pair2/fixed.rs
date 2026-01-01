use std::sync::{Arc, RwLock};
use std::thread;

struct Data {
    count: u32,
}

impl Data {
    // Safely increments the counter using a mutable borrow.
    fn increment(&mut self) {
        self.count = self.count.wrapping_add(1);
    }
}

struct Controller {
    data: Arc<RwLock<Data>>,
}

impl Controller {
    fn new() -> Self {
        Controller {
            data: Arc::new(RwLock::new(Data { count: 0 })),
        }
    }

    // Correct update: acquires a write lock to safely mutate the shared counter.
    fn update(&self) {
        let mut guard = self.data.write().unwrap();
        guard.increment();
    }

    fn get_count(&self) -> u32 {
        self.data.read().unwrap().count
    }
}

fn main() {
    let ctl = Controller::new();
    let shared_ctrl = Arc::new(ctl);

    let mut handles = vec![];
    let thread_count = 4;
    let iterations = 10_000;

    for _ in 0..thread_count {
        let ctl = Arc::clone(&shared_ctrl);
        handles.push(thread::spawn(move || {
            for _ in 0..iterations {
                ctl.update();
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = shared_ctrl.get_count();
    println!("Final count: {}", final_count);
}