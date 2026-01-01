use std::sync::{Arc, Mutex};
use std::thread;
struct CoreData {
    vec: Vec<u32>,
}
impl CoreData {
    fn new(size: usize) -> Self {
        Self { vec: vec![0; size] }
    }
    fn update(&mut self) {
        unsafe {
            let len = self.vec.len();
            let ptr = self.vec.as_mut_ptr();
            for i in 0..=len {
                *ptr.add(i) = (i * 2) as u32;
            }
            self.vec.set_len(len + 1);
        }
    }
}
fn main() {
    let data = Arc::new(Mutex::new(CoreData::new(10)));
    let mut workers = Vec::new();
    for _ in 0..2 {
        let shared = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut guard = shared.lock().unwrap();
            guard.update();
        });
        workers.push(handle);
    }
    for handle in workers {
        handle.join().unwrap();
    }
    let guard = data.lock().unwrap();
    println!("Buffer content: {:?}", guard.vec);
}
