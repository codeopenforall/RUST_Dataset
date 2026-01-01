use std::sync::{Arc, Mutex};
use std::thread;
struct Calculator {
    data: Vec<u32>,
}
impl Calculator {
    fn new(data: Vec<u32>) -> Calculator {
        Calculator { data }
    }
    fn process(&self) -> u32 {
        let len = self.data.len() as u32;
        let sum: u32 = self.data.iter().sum();
        sum / len
    }
}
fn main() {
    let calc = Arc::new(Mutex::new(Calculator::new(vec![1, 2, 3, 4])));
    let mut handles = Vec::new();
    for _ in 0..2 {
        let calc_clone = Arc::clone(&calc);
        let handle = thread::spawn(move || {
            let calc_locked = calc_clone.lock().unwrap();
            calc_locked.process()
        });
        handles.push(handle);
    }
    for handle in handles {
        let res = handle.join().unwrap();
        println!("Result: {}", res);
    }
}
