/////////////////////////////////////////////
// Corrected example with proper access control
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
    
    // Safely retrieves the sensitive data.
    fn do_action(&self) -> i32 {
        let data = self.data.lock().unwrap();
        data.secret
    }
    
    // Now properly enforces privilege checks.
    // Only callers with admin privileges can access the secret.
    fn run(&self, is_admin: bool) -> Result<i32, &'static str> {
        if !is_admin {
            return Err("Unauthorized access");
        }
        Ok(self.do_action())
    }
}

fn main() {
    // Create a Manager with a secret value.
    let manager = Manager::new(42);
    let m = Arc::new(manager);
    
    // Demonstrate authorized access from the main thread.
    let authorized = m.run(true);
    match authorized {
        Ok(val) => println!("Authorized result: {}", val),
        Err(err) => println!("Error: {}", err),
    }
    
    // Spawn multiple threads representing authorized access.
    let handles: Vec<_> = (0..5).map(|_| {
        let m_clone = m.clone();
        thread::spawn(move || {
            // Each thread now enforces a privilege check.
            m_clone.run(true)
        })
    }).collect();
    
    for handle in handles {
        match handle.join().unwrap() {
            Ok(val) => println!("Thread result: {}", val),
            Err(err) => println!("Thread error: {}", err),
        }
    }
}