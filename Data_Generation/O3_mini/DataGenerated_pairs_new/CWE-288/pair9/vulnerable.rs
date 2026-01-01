/*
This program simulates processing HTTP‐like requests concurrently.
It contains an authentication flaw where requests to the “/debug” path bypass the usual token checks.
An unsafe global flag (DEBUG_FLAG) is used to enable the alternate authentication path.
*/

use std::sync::{Arc, Mutex};
use std::thread;

static mut DEBUG_FLAG: bool = true; // Unsafe global flag controlling debug endpoint bypass

#[derive(Debug)]
enum Outcome {
    Granted(String),
    Refused(String),
}

#[derive(Debug)]
struct Inquiry {
    uri: String,
    header: Option<String>,
}

// Unsafe helper function that reads the global flag.
fn read_debug_flag() -> bool {
    unsafe { DEBUG_FLAG }
}

// Authentication routine: for the /debug endpoint, the unsafe flag allows bypassing normal auth.
fn verify(inq: &Inquiry) -> bool {
    // Flawed alternate path: if the endpoint is "/debug" and the flag is set, authentication is bypassed.
    if inq.uri == "/debug" && read_debug_flag() {
        return true;
    }
    // Normal endpoint: require header to exactly match "secret"
    inq.header.as_ref().map_or(false, |h| h == "secret")
}

fn execute(inq: Inquiry) -> Outcome {
    if verify(&inq) {
        Outcome::Granted(format!("Access granted for path: {}", inq.uri))
    } else {
        Outcome::Refused(format!("Access denied for path: {}", inq.uri))
    }
}

fn main() {
    // Simulate a set of incoming inquiries for different endpoints.
    let inquiries = vec![
        Inquiry { uri: "/debug".to_string(), header: None },
        Inquiry { uri: "/data".to_string(), header: Some("wrong".to_string()) },
        Inquiry { uri: "/data".to_string(), header: Some("secret".to_string()) },
    ];

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut threads = Vec::new();

    for inq in inquiries {
        let results = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let outcome = execute(inq);
            let mut guard = results.lock().unwrap();
            guard.push(outcome);
        });
        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }

    // Display outcomes.
    let guard = results.lock().unwrap();
    for outcome in guard.iter() {
        println!("{:?}", outcome);
    }
}