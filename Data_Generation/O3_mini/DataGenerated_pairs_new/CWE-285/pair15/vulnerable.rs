/////////////////////////////////////////////
// Vulnerable example with improper access control
/////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    secret: i32,
}

struct Manager {
    data: Arc<Mutex<Data>>,
}

impl Manager {
    fn new(secret: i32) -> Self {
        Self {
            data: Arc::new(Mutex::new(Data { secret })),
        }
    }
    
    // This method unsafely accesses the sensitive data.
    // It uses raw pointer operations inside unsafe blocks
    // and does not enforce any authorization checks.
    fn do_action(&self) -> i32 {
        let ptr: *const i32;
        unsafe {
            let data_ref = self.data.lock().unwrap();
            // Obtain raw pointer to the sensitive field.
            ptr = &data_ref.secret as *const i32;
        }
        // Unsafe dereference without further validation.
        unsafe { *ptr }
    }
    
    // Intended API to perform an action.
    // It erroneously ignores the provided admin flag.
    fn run(&self, _is_admin: bool) -> Result<i32, &'static str> {
        Ok(self.do_action())
    }
}

fn main() {
    // Create a Manager with a secret value.
    let manager = Manager::new(42);
    let m = Arc::new(manager);
    
    // Spawn multiple threads simulating concurrent access.
    // All threads pass 'false' to indicate unauthorized access.
    let handles: Vec<_> = (0..5).map(|_| {
        let m_clone = m.clone();
        thread::spawn(move || {
            // Bypasses authorization check and returns the secret.
            m_clone.run(false)
        })
    }).collect();
    
    for handle in handles {
        let result = handle.join().unwrap();
        match result {
            Ok(val) => println!("Result is: {}", val),
            Err(err) => println!("Error: {}", err),
        }
    }
}