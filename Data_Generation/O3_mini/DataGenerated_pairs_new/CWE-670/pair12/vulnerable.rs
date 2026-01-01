use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    flag: Arc<Mutex<u32>>,
}

impl Engine {
    fn new() -> Self {
        Engine { flag: Arc::new(Mutex::new(0)) }
    }

    // This function is intended to update the flag in a controlled manner based on the input value.
    // When passed the designated “trusted” value (1) it should use a safe branch, and for value (2)
    // it should use an alternative branch. However, due to an always-incorrect control flow,
    // even valid input (1) always results in an unsafe branch that uses flawed pointer arithmetic.
    fn process(&self, val: u32) -> Result<u32, &'static str> {
        match val {
            // This branch is correct: for value 2 we safely update the shared flag.
            2 => {
                let mut cnt = self.flag.lock().map_err(|_| "mutex poisoned")?;
                *cnt += 10;
                Ok(*cnt)
            },
            // BUG: The intended safe branch for valid input 1 is never reached,
            // because this arm (written as "1 | _") always fires.
            1 | _ => {
                // Unsafe block with flawed pointer arithmetic: the pointer is offset incorrectly.
                // This always happens regardless of input 1; it simulates an erroneous control flow.
                let base_ptr = &self.flag as *const Arc<Mutex<u32>>;
                unsafe {
                    // Incorrectly offset the pointer by 1 unit irrespective of the valid index.
                    // This has undefined behavior as it dereferences a pointer outside of its allocated object.
                    let bad_ptr = base_ptr.offset(1);
                    // Check is pointless because bad_ptr is not a null pointer in typical memory layouts.
                    if bad_ptr.is_null() {
                        return Err("null pointer encountered");
                    }
                    // Cast the miscomputed pointer back to a reference and attempt to lock.
                    let misplaced_ref: &Arc<Mutex<u32>> = &*bad_ptr;
                    let cnt = misplaced_ref.lock().map_err(|_| "mutex poisoned")?;
                    Ok(*cnt)
                }
            },
        }
    }
}

fn main() {
    let core = Engine::new();
    // For demonstration purposes we call process with 1 (which should be safe),
    // but due to flawed branch logic it always goes through the unsafe path.
    match core.process(1) {
        Ok(res) => println!("Outcome: {}", res),
        Err(err) => eprintln!("Error: {}", err),
    }
}