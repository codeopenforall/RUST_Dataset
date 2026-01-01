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

    // Correctly computes the percentage increase between the base and current values.
    // The subtraction is explicitly grouped to enforce the correct order: first subtract, then multiply by 100, and finally divide by base.
    unsafe fn calculate(&mut self) {
        let percent_ptr: *mut u32 = &mut self.percent;
        *percent_ptr = (self.current - self.base) * 100 / self.base;
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