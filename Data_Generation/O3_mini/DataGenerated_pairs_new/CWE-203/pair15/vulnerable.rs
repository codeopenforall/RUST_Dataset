////////////////////////////////////////////////////////////////
// Vulnerability Example Implementation (CWE-203)
//
// Note: This code intentionally differentiates errors for a valid
// user with a wrong password versus a non-existent user, leaking
// sensitive account existence information.
////////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;

#[derive(Debug)]
struct Record {
    name: String,
    secret_hash: u64, // Simplified hash value
}

struct Manager {
    records: Arc<Mutex<HashMap<String, Record>>>,
}

impl Manager {
    fn new() -> Self {
        let mut map = HashMap::new();
        // Adding one record with known credentials.
        map.insert("alice".to_string(), Record {
            name: "alice".to_string(),
            secret_hash: Manager::compute("SecureP@ssw0rd"),
        });
        Manager { records: Arc::new(Mutex::new(map)) }
    }

    fn compute(input: &str) -> u64 {
        // Using an unsafe block to mimic low-level operations.
        unsafe {
            let bytes = input.as_bytes();
            let mut hash: u64 = 0;
            for i in 0..bytes.len() {
                let ptr = bytes.as_ptr().offset(i as isize);
                hash = hash.wrapping_add(*ptr as u64);
            }
            hash
        }
    }

    // Authentication function that differentiates error messages.
    fn verify(&self, name: &str, secret: &str) -> Result<(), &'static str> {
        let records = self.records.lock().unwrap();
        if let Some(record) = records.get(name) {
            if record.secret_hash == Manager::compute(secret) {
                return Ok(());
            } else {
                // Vulnerability: returns a different error for an existing record.
                return Err("Incorrect secret for existing record");
            }
        } else {
            return Err("Record not found");
        }
    }
}

fn main() {
    let handler = Manager::new();
    // Launch concurrent tasks for demonstration.
    let shared = handler.records.clone();
    let threads: Vec<_> = (0..5).map(|i| {
        let instance = Manager { records: shared.clone() };
        thread::spawn(move || {
            unsafe {
                // Mimic concurrent unsafe operations.
                if i % 2 == 0 {
                    let num_ptr: *const u32 = &i;
                    let _ = *num_ptr;
                }
            }
            // A correct authentication; result is ignored.
            let _ = instance.verify("alice", "SecureP@ssw0rd");
        })
    }).collect();
    for t in threads {
        t.join().unwrap();
    }
    // End of main.
}