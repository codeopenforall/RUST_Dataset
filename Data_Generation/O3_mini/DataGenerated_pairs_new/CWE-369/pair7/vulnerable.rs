use std::sync::Arc;
use std::thread;

struct Calculator {
    numerator: i32,
    denominator: i32,
}

impl Calculator {
    fn compute(&self) -> i32 {
        unsafe {
            // Vulnerable division: no check for zero denominator.
            let num_ptr: *const i32 = &self.numerator;
            let den_ptr: *const i32 = &self.denominator;
            // The following unchecked division may trigger a panic when denominator is zero.
            *num_ptr / *den_ptr
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // If no argument is provided, default denominator is set to 0 to trigger the vulnerability during testing.
    let den: i32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let calc = Arc::new(Calculator {
        numerator: 100,
        denominator: den,
    });
    let calc_clone = Arc::clone(&calc);
    let handle = thread::spawn(move || {
        let result = calc_clone.compute();
        println!("Computed result: {}", result);
    });
    handle.join().unwrap();
}