/*
This Rust program implements a concurrent calculator that has been corrected to avoid
a divide-by-zero vulnerability. The division operation now performs a check to detect a
zero divisor before executing the unsafe division, and returns a defined safe value (0)
when the divisor is zero.
*/
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

trait Operation {
    fn compute(&self, dividend: i32, divisor: i32) -> i32;
}

struct Calculator;

impl Operation for Calculator {
    fn compute(&self, dividend: i32, divisor: i32) -> i32 {
        // Check if the divisor is zero to prevent unsafe division.
        if divisor == 0 {
            // Handle the error condition gracefully by returning 0.
            0
        } else {
            unsafe {
                dividend / divisor
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <divisor>", args[0]);
        return;
    }
    let divisor: i32 = args[1].parse().unwrap_or(1);

    let calc = Arc::new(Calculator);
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    // Spawn multiple threads executing the corrected computation concurrently.
    for i in 0..5 {
        let calc_clone = Arc::clone(&calc);
        let results_clone = Arc::clone(&results);
        let thread_divisor = divisor;
        handles.push(thread::spawn(move || {
            let res = calc_clone.compute(100, thread_divisor);
            let mut vec = results_clone.lock().unwrap();
            vec.push(res + i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_results = results.lock().unwrap();
    let sum: i32 = final_results.iter().sum();
    println!("Final computed sum: {}", sum);
}