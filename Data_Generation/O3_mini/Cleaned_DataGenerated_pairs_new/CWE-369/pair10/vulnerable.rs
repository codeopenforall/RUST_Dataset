use std::sync::{Arc, Mutex};
use std::thread;
use std::env;
trait Operation {
    fn compute(&self, divisor: i32) -> i32;
}
struct Engine;
impl Operation for Engine {
    fn compute(&self, divisor: i32) -> i32 {
        unsafe {
            let numerator = 10;
            let ptr = &numerator as *const i32;
            let result = *ptr / divisor;
            result
        }
    }
}
fn perform_calculation(divisor: i32) -> i32 {
    let engine = Engine;
    engine.compute(divisor)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].parse::<i32>().unwrap_or(1)
    } else {
        1
    };
    let shared_input = Arc::new(Mutex::new(input));
    let shared_calculation = Arc::new(Mutex::new(perform_calculation));
    let thread_input = Arc::clone(&shared_input);
    let thread_calc = Arc::clone(&shared_calculation);
    let handle = thread::spawn(move || {
        let val = *thread_input.lock().unwrap();
        let calc = thread_calc.lock().unwrap();
        let result = calc(val);
        println!("Result: {}", result);
    });
    handle.join().unwrap();
}
