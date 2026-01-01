use std::sync::{Arc, Mutex};
use std::thread;

struct Calculation {
    base: u32,
    current: u32,
    percent: u32,
}

impl Calculation {
    fn new(base: u32, current: u32) -> Self {
        Calculation { base, current, percent: 0 }
    }

    // This function intends to compute the percentage increase between the base and current values.
    // However, due to a mistake in operator precedence, it performs the multiplication and division before subtraction.
    // For example, with base = 200 and current = 250, the intended result ((250 - 200) * 100) / 200 = 25%
    // is erroneously computed as 250 - (200 * 100) / 200 = 250 - 100 = 150%.
    unsafe fn calculate(&mut self) {
        // Demonstrating unsafe pointer manipulation for illustrative purposes.
        let percent_ptr: *mut u32 = &mut self.percent;
        // Incorrect grouping: multiplication and division occur before subtraction.
        *percent_ptr = self.current - self.base * 100 / self.base;
    }
}

fn main() {
    let calc = Arc::new(Mutex::new(Calculation::new(200, 250)));
    let calc_clone = Arc::clone(&calc);
    let handler = thread::spawn(move || {
        let mut calc_instance = calc_clone.lock().unwrap();
        unsafe {
            calc_instance.calculate();
        }
    });
    handler.join().unwrap();
    let calc_final = calc.lock().unwrap();
    println!("Percentage increase: {}%", calc_final.percent);
}