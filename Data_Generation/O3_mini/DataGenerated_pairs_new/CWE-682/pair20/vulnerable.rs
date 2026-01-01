//////////////////////////////////////////////////////////////
// The code below contains an incorrect calculation vulnerability.
// The calculation in the unsafe block uses a misplaced operator precedence.
// Instead of computing numerator / (denom * scale) it computes 
// (numerator / denom) * scale leading to an order-of-operations error.
// Additionally, it uses unsafe pointer dereferencing and spawns a thread
// to mimic concurrent real-world usage.
//////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

// A Calculator struct performing scaling computations.
pub struct Calculator {
    pub scale: i32,
}

impl Calculator {
    // This unsafe function computes a result using raw pointer dereferencing.
    // Vulnerability: incorrect operator precedence in the calculation.
    pub unsafe fn compute(&self, numerator: i32, denominator: i32) -> i32 {
        // Access scale via raw pointer.
        let scale_ptr: *const i32 = &self.scale as *const i32;
        let scale_val: i32 = *scale_ptr;

        // Incorrect calculation: intends to compute numerator / (denom * scale)
        // but instead computes (numerator / denom) * scale.
        let result = (numerator / denominator) * scale_val;
        result
    }
}

// Function to run the calculation in a spawned thread.
fn launch_calculation(calc: Arc<Calculator>, num: i32, denom: i32, store: Arc<Mutex<i32>>) {
    let calc_clone = calc.clone();
    thread::spawn(move || {
        unsafe {
            let computed = calc_clone.compute(num, denom);
            let mut data = store.lock().unwrap();
            *data = computed;
        }
    })
    .join()
    .unwrap();
}

fn main() {
    // Example parameters: with scale=10, numerator=100, denominator=5
    // Intended correct result: 100/(5*10)=2.
    // Vulnerable code calculates: (100/5)*10 = 200.
    let calc = Arc::new(Calculator { scale: 10 });
    let result = Arc::new(Mutex::new(0));
    launch_calculation(calc, 100, 5, result.clone());
    let final_value = *result.lock().unwrap();
    println!("Computed result: {}", final_value);
}