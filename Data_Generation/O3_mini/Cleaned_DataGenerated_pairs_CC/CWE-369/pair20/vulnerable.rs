use std::env;
use std::process;
use std::thread;
use std::sync::Arc;
struct Calculator;
impl Calculator {
    fn execute(&self, denominator: i32) -> Result<i32, &'static str> {
        unsafe {
            Ok(100 / denominator)
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input: i32 = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    let calc = Calculator;
    let data = Arc::new(input);
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        *data_clone
    });
    let thread_val = handle.join().unwrap();
    let result = calc.execute(thread_val).unwrap();
    println!("Output: {}", result);
}
