//////////////////////////////////////////////
// Corrected Code for Improper Authorization
//////////////////////////////////////////////
use std::sync::Arc;
use std::cell::UnsafeCell;
use std::thread;

struct Data {
    key: String,
    privileged: bool,
}

// Global pointer holding the resource context.
static mut GLOBAL_DATA: Option<Arc<UnsafeCell<Data>>> = None;

// Initializes the global context.
fn initialize(key: &str, privileged: bool) -> Arc<UnsafeCell<Data>> {
    let d = Arc::new(UnsafeCell::new(Data { key: key.to_string(), privileged }));
    unsafe {
        GLOBAL_DATA = Some(d.clone());
    }
    d
}

// Performs a sensitive action that is restricted to authorized users.
// The corrected authorization check uses logical AND to ensure that only callers
// with a key matching the stored key AND with privileged == true may succeed.
fn sensitive_action(user_key: &str) -> Result<&'static str, &'static str> {
    unsafe {
        let data_ptr = GLOBAL_DATA.as_ref().unwrap().get();
        let data = &*data_ptr;
        // Proper Authorization: both conditions must be met for access.
        if user_key == data.key && data.privileged {
            Ok("Sensitive action performed")
        } else {
            Err("Unauthorized access")
        }
    }
}

fn main() {
    // FIX: Context is initialized with privileged set to true.
    // This ensures that only a properly authenticated call (with user_key "admin")
    // will succeed in performing the sensitive operation.
    let _ctx = initialize("admin", true);
    let user_key = "user"; // Non-admin user attempting access.
    
    let handles: Vec<_> = (0..4)
        .map(|_| {
            thread::spawn(move || {
                match sensitive_action(user_key) {
                    Ok(msg) => println!("{}", msg),
                    Err(err) => println!("Error: {}", err),
                }
            })
        })
        .collect();
    
    for h in handles {
        h.join().unwrap();
    }
}