//////////////////////////////////////
// Vulnerable Rust Code Sample
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

    // This function is intended to restrict access to the sensitive data.
    // However, it fails to check the caller's authorization credentials.
    unsafe fn run_operation(&self, _cred: Option<&Auth>) -> Result<i32, &'static str> {
        // VULNERABILITY: Missing authorization check.
        // The _cred parameter is completely ignored.
        let ptr: *const i32 = &self.sensitive;
        Ok(*ptr)
    }
}

fn main() {
    let res = Arc::new(Resource::new());
    let res_clone = Arc::clone(&res);
    let handle = thread::spawn(move || {
        unsafe {
            // Call the sensitive operation without any credentials.
            // Due to the missing check, this erroneously returns the sensitive value.
            res_clone.run_operation(None)
        }
    });
    match handle.join().unwrap() {
        Ok(val) => println!("Sensitive data: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}