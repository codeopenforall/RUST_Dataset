//////////////////////////////////////////////
/// A corrected implementation that preserves control flow
/// integrity using a Mutex-protected field within a struct.
/// The function pointer is stored within a lock (inside Regulator)
/// and can only be updated if a valid authorization token is provided.
/// This prevents unauthorized modifications, ensuring that the approved
/// branch is always executed.
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// The Regulator struct encapsulates the function pointer
// in a Mutex so that updates and accesses are serialized.
struct Regulator {
    op: Mutex<fn(i32) -> i32>,
}

impl Regulator {
    fn new() -> Self {
        // Initialize with the approved (safe) computation.
        Regulator { op: Mutex::new(approved) }
    }
    
    // Only allows updating the function pointer if the correct token is provided.
    fn update(&self, new_op: fn(i32) -> i32, token: &str) {
        if token == "secret_token" {
            let mut op_guard = self.op.lock().unwrap();
            *op_guard = new_op;
        }
    }
    
    // Executes the current function pointer in a controlled manner.
    fn execute(&self, input: i32) -> i32 {
        let op_guard = self.op.lock().unwrap();
        op_guard(input)
    }
}

// Approved computation: doubles the input.
fn approved(x: i32) -> i32 {
    x * 2
}

// Diverted computation: multiplies the input by 10.
fn diverted(x: i32) -> i32 {
    x * 10
}

// run_app creates a Regulator instance and spawns a thread attempting
// to update the function pointer with an unauthorized token. Since the token
// is invalid, the update does not occur and the approved branch is preserved.
fn run_app() -> i32 {
    let regulator = Arc::new(Regulator::new());
    let reg_clone = Arc::clone(&regulator);
    let handler = thread::spawn(move || {
        // Attempt to update with an incorrect token (update rejected).
        reg_clone.update(diverted, "wrong_token");
    });
    let _ = handler.join();
    
    // Execute the computation using the approved control flow.
    let result = regulator.execute(5);
    result
}

fn main() {
    let res = run_app();
    println!("Computed value: {}", res);
}