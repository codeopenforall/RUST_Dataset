use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
struct Data {
    value: i32,
}

impl Data {
    // This unsafe function simulates a low-level computation.
    // It correctly returns an error for negative inputs.
    unsafe fn compute(input: i32) -> Result<i32, &'static str> {
        let ptr = &input as *const i32;
        if *ptr < 0 {
            Err("input negative")
        } else {
            Ok(*ptr * 2)
        }
    }
}

// The corrected contract function now propagates errors properly.
// By using the ? operator inside each thread's closure, any error encountered (e.g. for negative inputs)
// is forwarded, allowing the caller to handle the error instead of silently substituting a default value.
pub fn run_contract(inputs: &[i32]) -> Result<Vec<i32>, &'static str> {
    let shared = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    for &input in inputs {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || -> Result<(), &'static str> {
            let res = unsafe { Data::compute(input)? };
            let mut vec = shared_clone.lock().unwrap();
            vec.push(res);
            Ok(())
        });
        handles.push(handle);
    }
    for handle in handles {
        if let Err(_) = handle.join().unwrap() {
            return Err("Thread failed processing input");
        }
    }
    let vec = shared.lock().unwrap();
    Ok(vec.clone())
}

fn main() {
    let inputs = vec![-5, 10, -3, 7];
    match run_contract(&inputs) {
        Ok(results) => println!("Aggregated Results: {:?}", results),
        Err(e) => {
            eprintln!("Error encountered: {}", e);
            std::process::exit(1);
        }
    }
}