use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
struct Data {
    threshold: u32,
}
impl Data {
    fn new(val: u32) -> Self {
        Data { threshold: val }
    }
    fn secure_update(&mut self, val: u32) {
        self.threshold = if val <= 10 { val } else { 10 };
    }
}
fn gather_input(input: u32) -> Arc<Mutex<Data>> {
    let shared = Arc::new(Mutex::new(Data::new(5)));
    {
        let shared_clone = Arc::clone(&shared);
        thread::spawn(move || {
            let mut data_guard = shared_clone.lock().unwrap();
            data_guard.secure_update(input);
        })
        .join()
        .unwrap();
    }
    shared
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input: u32 = if args.len() > 1 {
        args[1].parse().unwrap_or(5)
    } else {
        5
    };
    let shared = gather_input(input);
    let data = shared.lock().unwrap();
    assert!(data.threshold <= 10, "Threshold exceeded safe limit");
    println!("Threshold is safe: {}", data.threshold);
}
