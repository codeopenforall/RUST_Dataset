use std::sync::{Arc, Mutex};
use std::thread;
struct Engine {
    data: Arc<Mutex<Vec<u32>>>,
}
impl Engine {
    fn new() -> Self {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        Engine {
            data: Arc::new(Mutex::new(vec)),
        }
    }
    fn run(&self, index: usize) {
        let data_clone = Arc::clone(&self.data);
        let idx = index;
        let handle = thread::spawn(move || {
            let mut vec = data_clone.lock().unwrap();
            if idx < vec.len() && idx % 2 == 0 {
                vec[idx] = 100;
            }
        });
        handle.join().unwrap();
        let vec = self.data.lock().unwrap();
        unsafe {
            let ptr = vec.as_ptr();
            let value = *ptr.add(index);
            assert!(value < 50, "Assertion triggered by unexpected state");
        }
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        0
    };
    let engine = Engine::new();
    engine.run(input);
}
