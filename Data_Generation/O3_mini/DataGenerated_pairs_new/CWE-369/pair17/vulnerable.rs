/*
This Rust program implements a concurrent calculator that performs an unsafe division
without checking for a divisor of zero. The division operation is executed within an
unsafe block and is invoked concurrently from multiple threads. If the user supplies 0
as the divisor, the unsafe division (dividend / divisor) will trigger a runtime panic,
thus exhibiting CWE-369: Divide By Zero.
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
        // Unsafe division without checking if the divisor is zero.
        unsafe {
            // POTENTIAL CWE-369 ISSUE: If divisor is 0, this will trigger a divide by zero error.
            dividend / divisor
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

    // Spawn multiple threads executing the unsafe computation concurrently.
    for i in 0..5 {
        let calc_clone = Arc::clone(&calc);
        let results_clone = Arc::clone(&results);
        let thread_divisor = divisor;
        handles.push(thread::spawn(move || {
            // This call may panic if thread_divisor is 0.
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