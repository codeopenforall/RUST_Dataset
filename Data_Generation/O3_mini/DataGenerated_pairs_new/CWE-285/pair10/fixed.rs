//////////////////////////////////////
// Corrected Rust Code Sample
//////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct Resource {
    sensitive: i32,
}

struct Auth {
    level: u8,
}

impl Resource {
    fn new() -> Self {
        Resource { sensitive: 42 }
    }

    // The corrected function now performs proper validation of the caller's credentials.
    unsafe fn run_operation(&self, cred: Option<&Auth>) -> Result<i32, &'static str> {
        // FIX: Check that credentials exist and satisfy the minimum required level.
        if let Some(a) = cred {
            if a.level >= 5 {
                let ptr: *const i32 = &self.sensitive;
                return Ok(*ptr);
            }
        }
        Err("Unauthorized")
    }
}

fn main() {
    let res = Arc::new(Resource::new());
    let res_clone = Arc::clone(&res);
    let handle = thread::spawn(move || {
        unsafe {
            // In this scenario, no credentials are provided.
            // The proper check in run_operation() will reject the call.
            res_clone.run_operation(None)
        }
    });
    match handle.join().unwrap() {
        Ok(val) => println!("Sensitive data: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}