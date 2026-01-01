use std::sync::{Arc, Mutex};
use std::thread;
struct Data {
    value: Mutex<u32>,
}
impl Data {
    fn new(val: u32) -> Self {
        Data {
            value: Mutex::new(val),
        }
    }
    fn update(&self) {
        let mut lock = self.value.lock().unwrap();
        *lock += 1;
    }
    fn fetch(&self) -> u32 {
        *self.value.lock().unwrap()
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
