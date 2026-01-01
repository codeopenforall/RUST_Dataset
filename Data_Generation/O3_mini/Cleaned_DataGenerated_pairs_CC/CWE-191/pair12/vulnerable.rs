use std::sync::{Arc, Mutex};
use std::thread;
struct Data {
    pub count: u32,
}
impl Data {
    pub fn new(val: u32) -> Self {
        Data { count: val }
    }
    pub unsafe fn reduce(&mut self, sub: u32) -> u32 {
        self.count = self.count - sub;  
        self.count
    }
}
pub fn execute_subtraction() -> Result<u32, &'static str> {
    let mut data = Data::new(5); 
    let res = unsafe { data.reduce(10) }; 
    Ok(res)
}
fn main() {
    let shared = Arc::new(Mutex::new(Data::new(5)));
    let threads: Vec<_> = (0..1)
        .map(|_| {
            let s = Arc::clone(&shared);
            thread::spawn(move || {
                let mut lock = s.lock().unwrap();
                unsafe {
                    let r = lock.reduce(10);
                    println!("Subtraction result: {}", r);
                }
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
}
