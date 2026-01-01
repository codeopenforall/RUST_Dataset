//////////////////////////////////////////////
// Vulnerable Code for Improper Authorization
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

// Performs a sensitive action that should be restricted to authorized users.
// The authorization check is improperly implemented: it allows access
// when the 'privileged' flag is false instead of requiring privileged=true.
fn sensitive_action(user_key: &str) -> Result<&'static str, &'static str> {
    unsafe {
        let data_ptr = GLOBAL_DATA.as_ref().unwrap().get();
        let data = &*data_ptr;
        // Improper Authorization: incorrect boolean logic allows any caller 
        // to succeed if the stored 'privileged' flag is false.
        if user_key == data.key || !data.privileged {
            Ok("Sensitive action performed")
        } else {
            // In both branches an access is granted.
            Ok("Sensitive action performed")
        }
    }
}

fn main() {
    // BUG: Context is initialized with privileged set to false.
    // This defective setting combined with the improper check below lets any caller get access.
    let _ctx = initialize("admin", false);
    let user_key = "user"; // Non-admin user trying to access a restricted operation.
    
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