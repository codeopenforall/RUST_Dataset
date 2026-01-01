///////////////////////////////////////////////
// Vulnerable Code: CWE-200 - Exposure of Secrets
///////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Cred {
    user: String,
    secret: String,
}

impl Cred {
    fn new(user: &str, secret: &str) -> Self {
        Cred {
            user: user.to_owned(),
            secret: secret.to_owned(),
        }
    }
}

// Simulates processing that accidentally reveals sensitive info via logs.
// The unsafe block is used to mimic low-level memory operations.
fn process_data(data: &Arc<Mutex<Cred>>) -> String {
    let guard = data.lock().unwrap();
    unsafe {
        let ptr: *const Cred = &*guard;
        // Reading the memory without extra checks.
        let cred_copy = ptr.read();
        // The debug output inadvertently exposes the secret information.
        format!("{:?}", cred_copy)
    }
}

fn main() {
    let creds = Arc::new(Mutex::new(Cred::new("alice", "s3cr3t")));
    let creds_clone = Arc::clone(&creds);
    let handle = thread::spawn(move || {
        process_data(&creds_clone)
    });
    let output = handle.join().unwrap();
    // The printed output contains sensitive information.
    println!("{}", output);
}