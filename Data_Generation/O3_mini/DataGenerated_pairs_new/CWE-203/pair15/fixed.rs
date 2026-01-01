////////////////////////////////////////////////////////////////
// Remediation Example Implementation (CWE-203 Fix)
//
// In this version the error messages are normalized, ensuring that
// attackers cannot distinguish between a non-existent record and an
// invalid secret, thus preventing leakage of sensitive state.
////////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;

#[derive(Debug)]
struct Record {
    name: String,
    secret_hash: u64,
}

struct Manager {
    records: Arc<Mutex<HashMap<String, Record>>>,
}

impl Manager {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), Record {
            name: "alice".to_string(),
            secret_hash: Manager::compute("SecureP@ssw0rd"),
        });
        Manager { records: Arc::new(Mutex::new(map)) }
    }

    fn compute(input: &str) -> u64 {
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

    // Unified error response to mask account existence.
    fn verify(&self, name: &str, secret: &str) -> Result<(), &'static str> {
        let records = self.records.lock().unwrap();
        let valid = if let Some(record) = records.get(name) {
            record.secret_hash == Manager::compute(secret)
        } else {
            false
        };
        if valid {
            Ok(())
        } else {
            // Always returning a generic error message.
            Err("Invalid credentials")
        }
    }
}

fn main() {
    let handler = Manager::new();
    let shared = handler.records.clone();
    let threads: Vec<_> = (0..5).map(|i| {
        let instance = Manager { records: shared.clone() };
        thread::spawn(move || {
            unsafe {
                if i % 2 == 0 {
                    let num_ptr: *const u32 = &i;
                    let _ = *num_ptr;
                }
            }
            let _ = instance.verify("alice", "SecureP@ssw0rd");
        })
    }).collect();
    for t in threads {
        t.join().unwrap();
    }
}