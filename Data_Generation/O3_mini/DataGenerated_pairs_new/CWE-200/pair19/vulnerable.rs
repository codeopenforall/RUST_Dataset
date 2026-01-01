use std::sync::{Arc, Mutex};
use std::thread;

struct Secure {
    secret: String,
    logs: Arc<Mutex<Vec<String>>>,
}

impl Secure {
    fn new(secret: String) -> Secure {
        Secure { 
            secret, 
            logs: Arc::new(Mutex::new(Vec::new()))
        }
    }

    fn run_task(&self, index: usize) {
        // Using an unsafe block to simulate a low-level memory operation.
        // This block intentionally bypasses bounds checking.
        if index > self.secret.len() {
            unsafe {
                // Attempt an out-of-bound memory read.
                let _byte = *self.secret.as_ptr().offset(index as isize);
                // Vulnerability: The panic message logs the sensitive secret.
                panic!("Access error: secret leak: {}. Index: {}", self.secret, index);
            }
        } else {
            let msg = format!("Access OK at index {}.", index);
            let mut logs = self.logs.lock().unwrap();
            logs.push(msg);
        }
    }
}

fn main() {
    let secure = Arc::new(Secure::new("SuperSecretPassword".to_string()));
    let handles: Vec<_> = (0..3).map(|i| {
        let sec = secure.clone();
        thread::spawn(move || {
            sec.run_task(i);
        })
    }).collect();
    for handle in handles {
        let _ = handle.join();
    }
}