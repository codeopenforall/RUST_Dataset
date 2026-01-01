use std::sync::{Arc, Mutex};
use std::thread;
use std::env;
struct Data {
    value: usize,
}
impl Data {
    unsafe fn update(&mut self, new_value: usize) {
        let ptr = &mut self.value as *mut usize;
        *ptr = new_value;
    }
    fn check(&self) {
        assert!(self.value < 100, "Value too high");
    }
}
pub fn execute(input: usize) {
    let shared = Arc::new(Mutex::new(Data { value: 0 }));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let mut data = shared_clone.lock().unwrap();
        unsafe {
            data.update(input);
        }
        data.check();
    });
    handle.join().unwrap();
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        0
    };
    execute(input);
    println!("Execution completed.");
}
