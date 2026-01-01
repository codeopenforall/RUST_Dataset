//////////////////////////////////////////////
// Vulnerable Code - CWE-252: Unchecked Return Value
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

const MAX_LEN: usize = 5;

struct Resource {
    data: Mutex<String>,
}

impl Resource {
    // Attempts to update the resource state.
    // Returns an error if the input string length exceeds allowed maximum.
    fn update(&self, new_data: &str) -> Result<(), &'static str> {
        if new_data.len() > MAX_LEN {
            Err("Input too long")
        } else {
            let mut d = self.data.lock().unwrap();
            *d = new_data.to_string();
            Ok(())
        }
    }
}

// Simulates processing that is intended to update the resource.
// Vulnerability: the return value of update is ignored.
fn process(res: Arc<Resource>, input: &str) {
    // Unsafe block mimicking risky low-level code usage.
    unsafe {
        // Line 28: Ignored error from update.
        let _ = res.update(input);
    }

    // Spawn a thread that also ignores any error.
    let res_clone = Arc::clone(&res);
    let handle = thread::spawn(move || {
        unsafe {
            // Line 36: Another call ignoring returned error.
            let _ = res_clone.update(input);
        }
    });
    handle.join().unwrap();
}

fn main() {
    // Initial state is "init"
    let resource = Arc::new(Resource {
        data: Mutex::new("init".to_string()),
    });
    // Provide an input that triggers an error (length 6 > MAX_LEN).
    process(Arc::clone(&resource), "123456");

    // Even though update fails, the error is ignored.
    let final_state = resource.data.lock().unwrap();
    println!("Resulting state: {}", *final_state);
}