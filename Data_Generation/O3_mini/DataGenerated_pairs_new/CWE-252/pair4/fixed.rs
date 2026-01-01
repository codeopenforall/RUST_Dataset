//////////////////////////////////////////////
// Corrected Code - CWE-252 Mitigation: Checked Return Value
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

const MAX_LEN: usize = 5;

struct Resource {
    data: Mutex<String>,
}

impl Resource {
    // Attempts to update the resource state.
    // Returns an error if the input string exceeds allowed maximum.
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
// Correct behavior: error checking is implemented; if the update fails,
// a fallback using a truncated input is used.
fn process(res: Arc<Resource>, input: &str) {
    // Unsafe block included to mimic similar low-level context.
    let res_ref = &res;
    unsafe {
        // Line 68: Check the returned error and handle accordingly.
        if let Err(e) = res_ref.update(input) {
            // Fallback: Truncate the input to allowable length and try again.
            let truncated = &input[..MAX_LEN];
            res_ref.update(truncated)
                .expect("Fallback update failed");
        }
    }

    // In a separate thread, we also check the results rather than ignoring errors.
    let res_clone = Arc::clone(&res);
    let handle = thread::spawn(move || {
        unsafe {
            if let Err(e) = res_clone.update(input) {
                let truncated = &input[..MAX_LEN];
                res_clone.update(truncated)
                    .expect("Fallback update in thread failed");
            }
        }
    });
    handle.join().unwrap();
}

fn main() {
    // Initial state is "init"
    let resource = Arc::new(Resource {
        data: Mutex::new("init".to_string()),
    });
    // Provide an input that exceeds MAX_LEN.
    process(Arc::clone(&resource), "123456");

    // With proper error checking and fallback, the state is updated correctly.
    let final_state = resource.data.lock().unwrap();
    println!("Resulting state: {}", *final_state);
}