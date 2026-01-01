use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Default)]
struct Processor {
    threshold: i32,
}
impl Processor {
    fn new() -> Self {
        Processor { threshold: 0 }
    }
    fn process(&self, input: i32) -> Result<i32, &'static str> {
        unsafe {
            if input < 0 {
                Err("Negative value not allowed")
            } else {
                Ok(input + 42)
            }
        }
    }
}
fn main() {
    let proc = Arc::new(Mutex::new(Processor::new()));
    let mut handles = Vec::new();
    let inputs = vec![10, -5, 20, -1, 0];
    for &inp in &inputs {
        let proc_clone = Arc::clone(&proc);
        let handle = thread::spawn(move || {
            let p = proc_clone.lock().unwrap();
            match p.process(inp) {
                Ok(result) => println!("Input {} => Output {}", inp, result),
                Err(e)    => println!("Input {} => Error: {}", inp, e),
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
}
