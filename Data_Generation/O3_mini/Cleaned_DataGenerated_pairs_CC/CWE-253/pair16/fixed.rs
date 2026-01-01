use std::sync::{Arc, Mutex};
use std::thread;
pub struct Handler {
    data: Arc<Mutex<Vec<u32>>>,
}
impl Handler {
    pub fn new() -> Self {
        Handler {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub unsafe fn run(&self, input: u32) -> Result<u32, &'static str> {
        self.operate(input)
    }
    unsafe fn operate(&self, input: u32) -> Result<u32, &'static str> {
        let ret = self.unsafe_op(input);
        if ret == 0 {
            Ok(input.saturating_mul(2))
        } else {
            Err("operation failed")
        }
    }
    unsafe fn unsafe_op(&self, input: u32) -> i32 {
        if input % 2 == 0 { 0 } else { 1 }
    }
}
fn main() {
    let handler = Handler::new();
    let shared = Arc::new(handler);
    let mut threads = vec![];
    for i in 1..=4 {
        let proc = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            unsafe {
                match proc.run(i) {
                    Ok(val) => {
                        let mut agg = proc.data.lock().unwrap();
                        agg.push(val);
                    },
                    Err(e) => eprintln!("Thread input {} error: {}", i, e),
                }
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    let result = shared.data.lock().unwrap();
    println!("Aggregated result: {:?}", *result);
}
