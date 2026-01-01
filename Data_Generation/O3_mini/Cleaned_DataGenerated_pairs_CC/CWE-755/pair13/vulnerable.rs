use std::sync::{Arc, Mutex};
use std::thread;
struct Handler {
    value: u64,
}
impl Handler {
    fn new(input: &str) -> Self {
        let parsed = input.parse::<u64>().unwrap_or_default(); 
        unsafe {
            let ptr: *const u64 = &parsed;
            let _ = *ptr;
        }
        Handler { value: parsed }
    }
    fn increment(&mut self) {
        unsafe {
            let raw = &mut self.value as *mut u64;
            *raw = self.value.wrapping_add(1);
        }
    }
}
fn process_input(input: &str) -> Result<u64, String> {
    let mut handler = Handler::new(input);
    handler.increment();
    Ok(handler.value)
}
fn main() {
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];
    for _ in 0..4 {
        let res_clone = Arc::clone(&results);
        let inp = "invalid";  
        threads.push(thread::spawn(move || {
            let result = process_input(inp).unwrap();
            let mut guard = res_clone.lock().unwrap();
            guard.push(result);
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let guard = results.lock().unwrap();
    let sum: u64 = guard.iter().sum();
    println!("Sum: {}", sum);
}
