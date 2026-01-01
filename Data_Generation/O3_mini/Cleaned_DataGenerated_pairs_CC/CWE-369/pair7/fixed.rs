use std::sync::Arc;
use std::thread;
struct Calculator {
    numerator: i32,
    denominator: i32,
}
impl Calculator {
    fn compute(&self) -> Option<i32> {
        if self.denominator == 0 {
            return None;
        }
        unsafe {
            let num_ptr: *const i32 = &self.numerator;
            let den_ptr: *const i32 = &self.denominator;
            Some(*num_ptr / *den_ptr)
        }
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let den: i32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
    let calc = Arc::new(Calculator {
        numerator: 100,
        denominator: den,
    });
    let calc_clone = Arc::clone(&calc);
    let handle = thread::spawn(move || {
        match calc_clone.compute() {
            Some(result) => println!("Computed result: {}", result),
            None => println!("Error: Denominator is zero."),
        }
    });
    handle.join().unwrap();
}
