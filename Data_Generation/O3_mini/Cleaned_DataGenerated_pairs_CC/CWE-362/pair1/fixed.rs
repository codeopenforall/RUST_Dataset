use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
struct Data {
    counter: Arc<AtomicUsize>,
}
impl Data {
    fn new(initial: usize) -> Self {
        Data {
            counter: Arc::new(AtomicUsize::new(initial)),
        }
    }
    fn increment(&self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    }
    fn get(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }
}
pub fn execute_app() -> usize {
    let data = Data::new(0);
    let mut handles = vec![];
    for _ in 0..10 {
        let data_clone = Data { counter: data.counter.clone() };
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                data_clone.increment();
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    data.get()
}
fn main() {
    let final_val = execute_app();
    println!("Final counter: {}", final_val);
}
