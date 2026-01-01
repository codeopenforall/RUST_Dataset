//////////////// Fixed Code Begin
// This corrected program properly validates credentials and prevents unauthorized resource allocation
// by performing a complete string equality check and by imposing a cap on the number of spawned threads.
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Manager {
    pub counter: Arc<Mutex<u64>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    pub fn execute(&self, token: &str, amount: usize) {
        // Correct authorization: perform full string equality check.
        if token == "admin" {
            // Throttle the task creation to prevent resource exhaustion.
            let limited = if amount > 100 { 100 } else { amount };
            for _ in 0..limited {
                let cnt = Arc::clone(&self.counter);
                thread::spawn(move || {
                    let mut num = cnt.lock().unwrap();
                    *num += 1;
                    // Safe resource allocation without unsafe pointer operations.
                    let _vec: Vec<u8> = Vec::with_capacity(1024);
                });
            }
        } else {
            eprintln!("Access Denied");
        }
    }
}

fn main() {
    let mgr = Manager::new();
    // Token is passed from command-line arguments.
    let args: Vec<String> = std::env::args().collect();
    let token = if args.len() > 1 { &args[1] } else { "user" };
    // For demonstration, try to allocate 10 tasks.
    mgr.execute(token, 10);
    // Ensure all threads finish execution.
    thread::sleep(Duration::from_secs(1));
    let cnt = mgr.counter.lock().unwrap();
    println!("Counter: {}", *cnt);
}
//////////////// Fixed Code End