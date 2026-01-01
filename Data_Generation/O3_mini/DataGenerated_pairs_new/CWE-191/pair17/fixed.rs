/* 
This corrected example fixes the integer underflow vulnerability by using checked subtraction.
Before performing the subtraction, it verifies that the operation is valid.
If the subtraction would underflow, the code returns an error instead of modifying the state.
This ensures safe behavior even in a concurrent environment.
*/
use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    value: u32,
}

impl Data {
    fn new(val: u32) -> Self {
        Data { value: val }
    }
    
    // This method safely subtracts by checking for potential underflow.
    fn subtract(&mut self, amt: u32) -> Result<u32, &'static str> {
        // Fix: use checked_sub to verify that subtraction won't underflow.
        match self.value.checked_sub(amt) {
            Some(new_val) => {
                self.value = new_val;
                Ok(new_val)
            },
            None => Err("Underflow detected"),
        }
    }
}

fn main() {
    // Initialize with 0 so that subtracting 1 is tested for underflow.
    let data = Arc::new(Mutex::new(Data::new(0)));
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut d = data_clone.lock().unwrap();
        d.subtract(1)
    });
    let result = handle.join().unwrap();
    match result {
        Ok(val) => println!("Final value: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}