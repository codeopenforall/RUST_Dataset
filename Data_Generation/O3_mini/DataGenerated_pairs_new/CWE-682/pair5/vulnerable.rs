use std::sync::{Arc, Mutex};
use std::thread;

struct Calculator {
    data: Vec<u32>,
}

impl Calculator {
    fn new(data: Vec<u32>) -> Calculator {
        Calculator { data }
    }

    // This method attempts to compute an “average” by doing division on each element before summing.
    // It uses an unsafe block to perform raw pointer arithmetic.
    // The logic error is that dividing each element by the length (an integer division)
    // before the summation yields a truncated value for each element, resulting in an incorrect total.
    fn process(&self) -> u32 {
        let len = self.data.len() as u32;
        let mut result: u32 = 0;
        unsafe {
            let ptr = self.data.as_ptr();
            for i in 0..self.data.len() {
                // Vulnerability (CWE-682): Incorrect calculation by dividing each element
                // before summing, rather than summing first and then dividing.
                result += *ptr.add(i) / len;
            }
        }
        result
    }
}

fn main() {
    // Create a shared Calculator instance with sample data.
    let calc = Arc::new(Mutex::new(Calculator::new(vec![1, 2, 3, 4])));
    let mut handles = Vec::new();
    // Spawn two threads that independently perform the calculation.
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