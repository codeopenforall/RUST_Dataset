use std::sync::{Arc, Mutex};
use std::thread;
struct Data {
    pub count: u32,
}
impl Data {
    pub fn new(val: u32) -> Self {
        Data { count: val }
    }
    pub fn reduce(&mut self, sub: u32) -> Result<u32, &'static str> {
        if self.count < sub {
            return Err("Underflow error: subtraction would go below zero");
        }
        self.count -= sub;
        Ok(self.count)
    }
}
pub fn execute_subtraction() -> Result<u32, &'static str> {
    let mut data = Data::new(5); 
    data.reduce(10)
}
fn main() {
    let shared = Arc::new(Mutex::new(Data::new(5)));
    let threads: Vec<_> = (0..1)
        .map(|_| {
            let s = Arc::clone(&shared);
            thread::spawn(move || {
                let mut lock = s.lock().unwrap();
                match lock.reduce(10) {
                    Ok(val) => println!("Subtraction result: {}", val),
                    Err(e) => println!("Error: {}", e),
                }
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
}
