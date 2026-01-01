use std::{
    sync::{Arc, Mutex},
    thread,
};
fn calculate(a: u32, b: u32) -> u64 {
    let intermediate = a.wrapping_mul(b);
    let mut value = intermediate;
    unsafe {
        let ptr: *mut u32 = &mut value as *mut u32;
        *ptr = intermediate;
    }
    value as u64  
}
fn main() {
    let operand1: u32 = 100_000;
    let operand2: u32 = 50_000;
    let shared_result = Arc::new(Mutex::new(0u64));
    let shared_result_clone = Arc::clone(&shared_result);
    let handle = thread::spawn(move || {
        let result = calculate(operand1, operand2);
        unsafe {
            let mut lock = shared_result_clone.lock().unwrap();
            *lock = result;
        }
    });
    handle.join().unwrap();
    let final_value = *shared_result.lock().unwrap();
    println!("Result: {}", final_value);
}
