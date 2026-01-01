//////////////////////////////////////////////////////////////
// Corrected Code Sample for Information Exposure (CWE-203)  //
//////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Record {
    username: String,
    pass_hash: u64,
}

struct Service {
    users: Arc<Mutex<Vec<Record>>>,
}

impl Service {
    fn new() -> Self {
        Service {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn register(&self, name: &str, password: &str) {
        let hash = self.compute_hash(password);
        let rec = Record {
            username: name.to_string(),
            pass_hash: hash,
        };
        self.users.lock().unwrap().push(rec);
    }

    // Corrected: Always return a generic error message on authentication failure.
    fn authenticate(&self, name: &str, password: &str) -> Result<&'static str, &'static str> {
        let input_hash = self.compute_hash(password);
        let data = self.users.lock().unwrap();
        for rec in data.iter() {
            if rec.username == name && rec.pass_hash == input_hash {
                return Ok("Access granted");
            }
        }
        // Use a single generic message to avoid leaking whether the username exists.
        Err("Invalid credentials")
    }

    // Same unsafe hash implementation as before.
    fn compute_hash(&self, input: &str) -> u64 {
        let bytes = input.as_bytes();
        let mut sum = 0u64;
        unsafe {
            let mut ptr = bytes.as_ptr();
            for _ in 0..bytes.len() {
                sum = sum.wrapping_add(*ptr as u64);
                ptr = ptr.offset(1);
            }
        }
        sum
    }
}

fn main() {
    let svc = Service::new();
    svc.register("alice", "secret");

    // Spawn a thread to simulate concurrent authentication attempt.
    let svc_clone = Service {
        users: svc.users.clone(),
    };
    let handle = thread::spawn(move || {
        let res = svc_clone.authenticate("alice", "wrongpass");
        println!("Thread result: {:?}", res);
    });

    let res_main = svc.authenticate("nonexistent", "nopass");
    println!("Main thread result: {:?}", res_main);
    handle.join().unwrap();
}